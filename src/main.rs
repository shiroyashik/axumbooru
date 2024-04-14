use axum::{
    extract::DefaultBodyLimit, middleware::from_extractor, routing::{get, post}, Router
};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower_http::trace::TraceLayer;
use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::{Arc, Mutex}};
use log::{debug, info, trace};

// Config
pub mod config;
pub use config::Config;

// Api
pub mod api;
pub use api::info;

// Error
pub mod error;
pub use error::{ErrorStruct, Result};

// Auth
pub mod auth;
pub use auth::RequireAuth;

#[derive(Debug)]
pub struct AppState {
    db: DatabaseConnection,
    config: Config,
    uploads: Mutex<HashMap<String, api::data::Uploads>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    debug!("Current dir: {:?}", std::env::current_dir());

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up connection pool
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Trace);

    let state = Arc::new(AppState {
        db: Database::connect(opt)
            .await
            .expect("Database connection error!"),
        config: Config::parse(PathBuf::from_str("booruconfig.toml").unwrap()),
        uploads: Mutex::new(HashMap::new()),
    });

    debug!("State ready!");
    trace!("Data:\n{:?}", state);

    let app = Router::new()
        .route("/uploads", post(api::data::upload).layer(DefaultBodyLimit::max(1073741824))) // 1 GB

        .route("/test", get(api::test::test))
        .route("/posts/", get(api::post::list_of_posts))
        .route("/post/:id", get(api::post::get_post_by_id))
        .route("/user/:user", get(api::user::get_user))
        .route("/user-tokens/:user", get(api::usertoken::list_usertokens))
        .route("/user-token/:user", post(api::usertoken::create_usertoken))
        .route("/users", post(api::user::create_user))
        .route_layer(from_extractor::<RequireAuth>()) // Auth, functions lower doesn't require it.
        .route("/info", get(api::info::server_info))
        .fallback_service(api::data::data_static())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6667")
        .await
        .unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    info!("Serve stopped. Closing...");
}

// async fn handler() -> Result<(), AppError> {
//     try_thing()?;
//     Ok(())
// }
// fn try_thing() -> Result<(), anyhow::Error> {
//     anyhow::bail!("epic fail!")
// }

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
//
// #[derive(Serialize)]
// struct ApiErrorResponse {
//     name: String,
//     title: String,
//     description: String,
// }
//
// // Make our own error that wraps `anyhow::Error`.
// struct AppError(anyhow::Error);
//
// // Tell axum how to convert `AppError` into a response.
// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             [("Content-Type", "application/json")],
//             serde_json::to_string_pretty(&ApiErrorResponse {
//                 name: "ValidationError".to_string(),
//                 title: self.0.to_string(),
//                 description: "Error".to_string(),
//             })
//             .unwrap(),
//         ).into_response()
//     }
// }
//
// // This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// // `Result<_, AppError>`. That way you don't need to do that manually.
// impl<E> From<E> for AppError
// where
//     E: Into<anyhow::Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }
