use super::*;

#[derive(Deserialize)]
pub struct NewInfo {
    pub search: String,
}

pub fn new(info: Query<NewInfo>) -> HttpResponse {
    let location = if info.search == "".to_string() {
        "/".to_string()
    } else {
        format!("/search/{}", info.search)
    };
    HttpResponse::Found().header("location", location).finish()
}

#[derive(Deserialize)]
pub struct IndexInfo {
    pub page: Option<i64>,
}

pub fn index(
    (path, info, state): (Path<(String)>, Query<IndexInfo>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    let data = SearchIndex {
        page: info.page.to_owned(),
        search_content: path.to_string(),
    };
    to_template!(state, data, NotFound)
}