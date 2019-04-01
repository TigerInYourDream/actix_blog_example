#[macro_use]
pub mod marco;
pub mod article;
pub mod home;
pub mod search;
pub mod users;

use time::Duration;

use crate::jwt;
use actix_web::middleware::identity::*;
use actix_web::*;
use futures::Future;
use serde_derive::{Deserialize, Serialize};

pub use self::marco::*;
use crate::handler::*;
use crate::routes::AppState;

use db::models::*;
use view::views::*;
