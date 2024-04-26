use axum::{
    extract::FromRequestParts,
    http::{header, StatusCode, request::Parts},
    async_trait,
};
use log::debug;

use crate::UserRank;

pub struct RequireAuth {
    pub has_authorization: bool,
    pub user: Option<User>,
}

impl Default for RequireAuth {
    fn default() -> Self {
        Self {
            has_authorization: false,
            user: None,
        }
    }
}

pub struct User {
    pub id: i32,
    pub name: String,
    pub rank: UserRank,
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) => {
                let auth_header = auth_header.split(' ').collect::<Vec<&str>>();
                debug!("Auth: {auth_header:?}");
                Ok(Self::default())
            },
            _ => { // Doesn't have AUTHORIZATION
                debug!("Don't have Auth: {auth_header:?}");
                Ok(Self::default())
            },
        }
    }
}

fn token_is_valid(token: &str) -> bool {
    // ...
    true
}

fn password_is_valid(password_hash: &str, password_salt: &str) -> bool {
    true
}