///Marco for Actor system; Actor send message here; So the message can pass to the handle(Handle for Actor System)
use super::*;

macro_rules! to_template {
    ($state: expr, $t: expr, $e: ident) => {{
        $state
            .pool
            .send($t)
            .from_err()
            .and_then(|res| match res {
                //use temple here
                Ok(s) => Ok(HttpResponse::Ok().content_type("text/html").body(s)),
                Err(_) => Ok(HttpResponse::$e().into()),
            }).responder()
    }};
}

macro_rules! to_location {
    ($state: expr, $t: expr, $e: ident, $l: expr) => {{
        $state
            .pool
            .send($t)
            .from_err()
            .and_then(|res| match res {
                Ok(_) => Ok(HttpResponse::Found()
                    .header(http::header::LOCATION, $l)
                    .finish()),
                Err(_) => Ok(HttpResponse::$e().into()),
            }).responder()
    }};
}

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

macro_rules! get_user_id {
    ($req: expr) => {
        $req.identity()
            .and_then(|to| {
                let token = jwt::decode::<JWTClaims>(
                    &to,
                    JWT::secret().as_ref(),
                    &jwt::Validation::default(),
                );
                println!("{:?}", token);
                if let Ok(t) = token {
                    Some(t.claims.user_id)
                } else {
                    None
                }
            }).unwrap_or("".to_string())
    };
}
