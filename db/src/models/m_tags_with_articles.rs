use super::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "tags_with_articles"]
pub struct TagWithArticle {
    pub id: Uuid,
    pub tag_id: Uuid,
    pub article_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "tags_with_articles"]
pub struct NewTagWithArticle {
    pub tag_id: Uuid,
    pub article_id: Uuid,
}

#[derive(AsChangeset)]
#[table_name = "tags_with_articles"]
pub struct UpdateTagWithArticle {
    pub id: Uuid,
    pub tag_id: Uuid,
    pub article_id: Uuid,
}
