use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use dotenvy::dotenv;
use serde::{self, Serialize};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axumbooru=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/", get(handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6667")
        .await
        .unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("epic fail!")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    info!("Terminate signal received");
}

// #[allow(dead_code)]
// #[derive(Serialize, Debug, thiserror::Error)]
// enum ApiErrorTypes {
//     MissingRequiredFileError,
//     MissingRequiredParameterError,
//     InvalidParameterError,
//     IntegrityError,
//     SearchError,
//     AuthError,
//     PostNotFoundError,
//     PostAlreadyFeaturedError,
//     PostAlreadyUploadedError,
//     InvalidPostIdError,
//     InvalidPostSafetyError,
//     InvalidPostSourceError,
//     InvalidPostContentError,
//     InvalidPostRelationError,
//     InvalidPostNoteError,
//     InvalidPostFlagError,
//     InvalidFavoriteTargetError,
//     InvalidCommentIdError,
//     CommentNotFoundError,
//     EmptyCommentTextError,
//     InvalidScoreTargetError,
//     InvalidScoreValueError,
//     TagCategoryNotFoundError,
//     TagCategoryAlreadyExistsError,
//     TagCategoryIsInUseError,
//     InvalidTagCategoryNameError,
//     InvalidTagCategoryColorError,
//     TagNotFoundError,
//     TagAlreadyExistsError,
//     TagIsInUseError,
//     InvalidTagNameError,
//     InvalidTagRelationError,
//     InvalidTagCategoryError,
//     InvalidTagDescriptionError,
//     UserNotFoundError,
//     UserAlreadyExistsError,
//     InvalidUserNameError,
//     InvalidEmailError,
//     InvalidPasswordError,
//     InvalidRankError,
//     InvalidAvatarError,
//     ProcessingError,
//     ValidationError,
// }

#[derive(Serialize)]
struct ApiErrorResponse {
    name: String,
    title: String,
    description: String
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self.0)).into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}