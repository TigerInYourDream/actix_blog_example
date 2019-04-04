pub mod view_article;
pub mod view_home;
pub mod view_search;
pub mod view_users;

pub use self::view_article::*;
pub use self::view_home::*;
pub use self::view_search::*;
pub use self::view_users::*;

pub use askama::Template;
use db::models::*;
