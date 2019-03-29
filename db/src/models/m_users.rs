use super::*;

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub user_email: String,
    pub pass_word: String,
    pub salt: String,
    pub nick_name: String,
    pub role_level: i16,
    // 9999 root_admin
    pub sign_up_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_email: String,
    pub pass_word: String,
    pub salt: String,
    pub nick_name: String,
    pub role_level: i16,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: Uuid,
    pub pass_word: String,
    pub nick_name: String,
    pub role_level: i16, // 9999 root_admin
}