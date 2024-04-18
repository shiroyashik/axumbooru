pub mod post;
pub use post::{Mutation as PostMutation, Query as PostQuery};

pub mod user;
pub use user::{Mutation as UserMutation, Query as UserQuery};

pub mod usertoken;
pub use usertoken::{Mutation as UserTokenMutation, Query as UserTokenQuery};
