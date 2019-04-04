use super::*;

#[derive(Template)]
#[template(path = "search_index.html")]
pub struct ViewSearchIndex {
    pub page_count: i64,
    pub page_number: i64,
    pub articles: Vec<Article>,
    pub search_content: String,
}
