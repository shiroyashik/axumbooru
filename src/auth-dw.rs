use axum::async_trait;
use password_auth::verify_password;
use sea_orm::DatabaseConnection;
use serde::{Serialize, Deserialize};
use axum_login::{self, AuthUser, AuthnBackend, UserId};
use tokio::task;

//
// FOR DELETE! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! ! !
//

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    id: i32,
    pub name: String,
    password: String,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.name)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: DatabaseConnection,
}

impl Backend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;
    
    async fn authenticate(
        &self,
        creds: Self::Credentials
    ) -> Result<Option<Self::User>, Self::Error> {
        let raw_user = service::UserQuery::find_user_credentials_by_name(&self.db, &creds.username).await?.unwrap();
        let user = Some(Self::User { id: raw_user.id, name: raw_user.name, password: raw_user.password_hash });
        task::spawn_blocking(|| {
            // We're using password-based authentication--this works by comparing our form
            // input with an argon2 password hash.
            Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let raw_user = service::UserQuery::find_user_credentials_by_id(&self.db, *user_id).await?.unwrap();
        let user = Some(Self::User { id: raw_user.id, name: raw_user.name, password: raw_user.password_hash });
        Ok(user)
    }
}