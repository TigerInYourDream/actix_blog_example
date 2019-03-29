use super::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "pages"]
pub struct Page {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "pages"]
pub struct NewPage {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(AsChangeset)]
#[table_name = "pages"]
pub struct UpdatePage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}