use crate::jwt;
use actix_web::middleware::identity::*;
use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, Result};

use crate::api::*;
use db::models::*;
use time::Duration;

// jwt
pub struct JWT {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub user_id: String,
    pub refresh: usize,
    pub exp: usize,
}

impl JWT {
    pub fn secret() -> &'static str {
        "666"
    }

    pub fn token(&self) -> Result<String, jwt::errors::Error> {
        let claims = JWTClaims {
            user_id: self.user_id.to_owned(),
            refresh: NaiveDateTime::from_timestamp(Local::now().timestamp(), 6)
                .checked_add_signed(Duration::hours(6))
                .unwrap()
                .timestamp() as usize,
            exp: NaiveDateTime::from_timestamp(Local::now().timestamp(), 6)
                .checked_add_signed(Duration::hours(60))
                .unwrap()
                .timestamp() as usize,
        };
        let token = jwt::encode(&jwt::Header::default(), &claims, JWT::secret().as_ref())?;
        Ok(token)
    }
}

pub struct JWTMiddleWare; // <- my middleware

/// Middleware implementation, middlewares are generic over application state,
/// so you can access state with `HttpRequest::state()` method.
impl<S> Middleware<S> for JWTMiddleWare {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.identity().and_then(|to| {
            let token = jwt::decode::<JWTClaims>(&to, "666".as_ref(), &jwt::Validation::default());
            println!("This is my token : {:?}", token);
            if let Ok(t) = token {
                let refresh = t.claims.refresh;
                let now =
                    NaiveDateTime::from_timestamp(Local::now().timestamp(), 6).timestamp() as usize;
                if now > refresh {
                    let jwt = JWT {
                        user_id: t.claims.user_id.to_owned(),
                    };
                    req.remember(jwt.token().unwrap());
                };
                Some(())
            } else {
                req.forget();
                None
            }
        });
        Ok(Started::Done)
    }
}