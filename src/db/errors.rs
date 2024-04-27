#[derive(thiserror::Error, Debug)]
#[error("Something went wrong.")]
pub struct DatabaseError {
    #[from]
    source: anyhow::Error,
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserError {
    #[error("There is no user with {name:?} as name.")]
    UserNotFound {
        name: String,
        #[source]
        source: DatabaseError
    },
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(thiserror::Error, Debug)]
pub enum DeleteUserTokenError {
    #[error("There is no user_token with {token:?} as token.")]
    TokenNotFound {
        token: String,
        #[source]
        source: DatabaseError
    },
    #[error("Token user_id and user doesn't match!")]
    TokenUserIdDontMatch,
    #[error("Something went wrong.")]
    DatabaseError(#[from] DatabaseError),
}