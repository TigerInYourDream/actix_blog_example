/// model for categories
use super::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[table_name = "categories"]
pub struct Category {
    pub id: Uuid,
    pub super_id: Option<Uuid>,
    pub cat_name: String,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub super_id: Option<Uuid>,
    pub cat_name: String,
}

#[derive(AsChangeset)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub id: Uuid,
    pub super_id: Option<Uuid>,
    pub cat_name: String,
}