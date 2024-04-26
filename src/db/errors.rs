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