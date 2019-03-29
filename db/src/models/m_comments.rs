use super::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "comments"]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub nick_name: String,
    pub contact_address: String,
    pub content: String,
    pub create_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub article_id: Uuid,
    pub nick_name: String,
    pub contact_address: String,
    pub content: String,
    pub create_time: NaiveDateTime,
}