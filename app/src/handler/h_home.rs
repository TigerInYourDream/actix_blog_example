use super::*;

#[derive(GenMessage)]
pub struct HomeIndex {
    pub page: Option<i64>,
}

impl Handler<HomeIndex> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: HomeIndex, _: &mut Self::Context) -> Self::Result {
        let conn: &DbConn = &self.0.get().unwrap();
        let page_limit = 2i64;
        let (page_number, offset) = if let Some(a) = msg.page {
            (a, (a - 1) * page_limit)
        } else {
            (1i64, 0i64)
        };
        let items = articles::table
            .order(articles::create_time.desc())
            .offset(offset)
            .limit(page_limit)
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let articles_count = articles::table
            .count()
            .get_result::<i64>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let page_count = if articles_count % page_limit == 0 {
            articles_count / page_limit
        } else {
            (articles_count / page_limit) + 1
        };
        let articles = ViewHomeIndex {
            page_count: page_count,
            page_number: page_number,
            articles: items,
        };
        Ok(articles.render().unwrap())
    }
}