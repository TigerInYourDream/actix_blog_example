use super::*;

#[derive(GenMessage, Deserialize, Clone)]
pub struct LoginForm {
    pub user_email: String,
    pub pass_word: String,
}

impl Handler<LoginForm> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: LoginForm, _: &mut Self::Context) -> Self::Result {
        let conn: &DbConn = &self.0.get().unwrap();
        let items = users::table
            .filter(users::user_email.eq(msg.user_email))
            .first::<User>(conn)
            .map_err(error::ErrorInternalServerError)?;

        let pass_word = format!("{}{}", msg.pass_word, items.salt);
        let pass_word = format!("{:x}", md5::compute(pass_word.as_bytes()));
        println!("{}", pass_word);
        println!("{}", items.pass_word);
        if pass_word != items.pass_word {
            return Err(error::ErrorInternalServerError("password wrong"));
        };
        Ok(items.id.to_string())
    }
}