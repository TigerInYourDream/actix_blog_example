use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use actix_web::middleware::identity::*;
use actix_web::{fs, http::Method, middleware, App};
use time::Duration;

use crate::api::*;
use crate::middlewares::*;
use db::{DbConnecting, DbPool};

pub struct DbExecutor(pub DbPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// so we can share The AppState(DbExecutor) in App, we can use it in method
pub struct AppState {
    pub pool: Addr<DbExecutor>,
}

pub fn app() -> App<AppState> {
    let pool = DbConnecting::establish_pool();
    // Start 6 parallel db executors
    let addr = SyncArbiter::start(6, move || DbExecutor(pool.clone()));

    App::with_state(AppState { pool: addr.clone() })
        .middleware(middleware::Logger::default())
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(&[0; 32])
                .name("auth")
                .max_age(Duration::hours(24 * 7))
                .secure(false),
        ))
        .middleware(JWTMiddleWare)
        .resource("/", |r| r.method(Method::GET).with(home::index))
        .resource("/search", |r| {
            r.method(Method::GET).with(search::new);
        })
        .resource("/search/{content}", |r| {
            r.method(Method::GET).with(search::index);
        })
        .resource("/articles", |r| {
            r.method(Method::GET).with(article::index);
            r.method(Method::POST).with(article::create);
        })
        .resource("/articles/new", |r| {
            r.method(Method::GET).with(article::new);
        })
        .resource("/articles/{id}", |r| {
            r.method(Method::GET).with(article::show);
        })
        .resource("/articles/{id}/put", |r| {
            r.method(Method::POST).with(article::update);
        })
        .resource("/articles/{id}/delete", |r| {
            r.method(Method::POST).with(article::destroy);
        })
        .resource("/articles/{id}/edit", |r| {
            r.method(Method::GET).with(article::edit)
        })
        .resource("/users/login", |r| {
            r.method(Method::GET).with(users::login_show);
            r.method(Method::POST).with(users::login);
        })
        .resource("/users/logout", |r| {
            r.method(Method::POST).with(users::logout)
        })
        .handler("/resource",
                 fs::StaticFiles::new("resource")
                     .unwrap()
                     .show_files_listing())
}