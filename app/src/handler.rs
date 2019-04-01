mod h_article;
mod h_home;
mod h_search;
mod h_users;

pub use self::h_article::*;
pub use self::h_home::*;
pub use self::h_search::*;
pub use self::h_users::*;

use actix_web::actix::{Handler, Message};
use actix_web::{error, Error, Result};
use serde_derive::Deserialize;
use proc_for_actix::GenMessage;

use crate::routes::DbExecutor;

use db::models::*;
use db::schema::*;
use db::DbConn;
use view::views::*;