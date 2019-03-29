use super::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "tags"]
pub struct Tag {
    pub id: Uuid,
    pub tag_name: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub tag_name: String,
}

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct UpdateTag {
    pub id: Uuid,
    pub tag_name: String,
}