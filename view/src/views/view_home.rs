use super::*;

#[derive(Template)]
#[template(path = "home_index.html")]
pub struct ViewHomeIndex {
    pub page_count: i64,
    pub page_number: i64,
    pub articles: Vec<Article>,
}