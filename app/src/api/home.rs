use super::*;

#[derive(Deserialize)]
pub struct Info {
    pub page: Option<i64>,
}

pub fn index((info, state): (Query<Info>, State<AppState>)) -> FutureResponse<HttpResponse> {
    let data = HomeIndex {
        page: info.page.to_owned(),
    };
    to_template!(state, data, NotFound)
}