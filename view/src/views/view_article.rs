use super::*;

#[derive(Template)]
#[template(path = "articles_show.html", escape = "none")]
pub struct ViewArticle {
    pub category_id: Uuid,
    pub title: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Template)]
#[template(path = "articles_edit.html")]
pub struct ViewArticleEdit {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub release_status: i16,
    pub title: String,
    pub content: String,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: NaiveDateTime,
}

#[derive(Template)]
#[template(path = "articles_new.html")]
pub struct ViewArticleNew {}

#[derive(Template)]
#[template(path = "articles_index.html")]
pub struct ViewArticleIndex {
    pub articles: Vec<Article>,
}