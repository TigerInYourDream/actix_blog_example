use super::*;

pub fn login_show(_state: State<AppState>) -> HttpResponse {
    let s = UserLoginShow {}.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

pub fn login(
    (form, state, req): (Form<LoginForm>, State<AppState>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    let data = form.clone();
    state
        .pool
        .send(data)
        .from_err()
        .and_then(move |res| match res {
            Ok(_id) => {
                let jwt = JWT { user_id: _id };
                req.remember(jwt.token().unwrap());
                Ok(HttpResponse::Found().header("location", "/").finish())
            }
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}

pub fn logout(req: HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Found().header("location", "/").finish()
}