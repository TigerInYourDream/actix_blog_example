use super::*;

pub fn index(state: State<AppState>) -> FutureResponse<HttpResponse> {
    let data = ArticlesIndex {};
    to_template!(state, data, NotFound)
}

pub fn show((info, state): (Path<(String)>, State<AppState>)) -> FutureResponse<HttpResponse> {
    let data = ShowArticle {
        id: info.to_string(),
    };
    to_template!(state, data, NotFound)
}

pub fn new(_state: State<AppState>) -> HttpResponse {
    let s = ViewArticleNew {}.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(s)
}

pub fn create(
    (form, state, req): (Form<NewArticleForm>, State<AppState>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    let mut data = form.clone();
    let user_id = get_user_id!(req);
    data.user_id = user_id;
    let location = format!("/articles");
    to_location!(state, data, NotFound, location)
}

pub fn edit(
    (info, state, req): (Path<(String)>, State<AppState>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    let user_id = get_user_id!(req);
    let data = EditArticle {
        id: info.to_string(),
        user_id: user_id,
    };
    to_template!(state, data, NotFound)
}

pub fn update(
    (form, info, state, req): (
        Form<UpdateArticleForm>,
        Path<(String)>,
        State<AppState>,
        HttpRequest<AppState>,
    ),
) -> FutureResponse<HttpResponse> {
    let mut data = form.clone();
    let user_id = get_user_id!(req);
    data.id = info.to_string();
    data.user_id = user_id;
    let location = format!("/articles/{}", info.to_string());
    to_location!(state, data, NotFound, location)
}

pub fn destroy(
    (info, state, req): (Path<(String)>, State<AppState>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    let user_id = get_user_id!(req);
    let data = DeleteArticle {
        id: info.to_string(),
        user_id: user_id,
    };
    let location = format!("/articles");
    to_location!(state, data, NotFound, location)
}
