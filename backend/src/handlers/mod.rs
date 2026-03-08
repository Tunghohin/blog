pub mod auth;
pub mod comments;
pub mod posts;

pub use auth::{login, register};
pub use comments::{list_comments, create_comment, delete_comment};
pub use posts::{list_posts, get_post, create_post, update_post, delete_post};
