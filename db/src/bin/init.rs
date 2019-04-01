///init the root_user
extern crate db;

use db::models::*;
use db::schema::*;
use db::DbConnecting;

extern crate md5;

fn insert_root_user() -> () {
    let conn = DbConnecting::establish_connection();

    let pass = "000".to_string();
    let pass = format!("{:x}", md5::compute(pass.as_bytes()));
    let salt = "792316348".to_string();
    let pass_word = format!("{}{}", pass, salt);
    let pass_word = format!("{:x}", md5::compute(pass_word.as_bytes()));

    let root_user = NewUser {
        user_email: "123@123.com".to_string(),
        pass_word: pass_word,
        salt: salt,
        nick_name: "nick_name".to_string(),
        role_level: 9999i16,
    };

    diesel::insert_into(users::table)
        .values(&root_user)
        .get_result::<User>(&conn)
        .expect("Error saving new user");

    println!("init inserted: User");
}

fn insert_root_cat() -> () {
    let conn = DbConnecting::establish_connection();

    let root_cat = NewCategory {
        super_id: None,
        cat_name: "Default Category".to_string(),
    };

    diesel::insert_into(categories::table)
        .values(&root_cat)
        .get_result::<Category>(&conn)
        .expect("Error saving new Category");

    println!("init inserted: Category");
}

fn insert_root_article() -> () {
    let conn = DbConnecting::establish_connection();

    let user = users::table
        .filter(users::user_email.eq("123@123.com"))
        .first::<User>(&conn)
        .expect("Error loading users");

    let cat = categories::table
        .filter(categories::cat_name.eq("Default Category"))
        .first::<Category>(&conn)
        .expect("Error loading Category");

    let root_article = NewArticle {
        user_id: user.id,
        category_id: cat.id,
        title: "Hello World !".to_string(),
        content: "This is the first blog".to_string(),
        release_status: 100i16,
    };

    diesel::insert_into(articles::table)
        .values(&root_article)
        .get_result::<Article>(&conn)
        .expect("Error saving new Article");

    println!("init inserted: Article");
}

fn main() {
    let conn = DbConnecting::establish_connection();

    let res_vec = users::table
        .filter(users::user_email.eq("123@123.com"))
        .limit(1)
        .load::<User>(&conn)
        .expect("Error loading users");

    if res_vec.len() < 1 {
        insert_root_user();
    } else {
        println!("Root user has been added");
    };

    let cat_vec = categories::table
        .filter(categories::cat_name.eq("Default Category"))
        .limit(1)
        .load::<Category>(&conn)
        .expect("Error loading Category");

    if cat_vec.len() < 1 {
        insert_root_cat();
    } else {
        println!("Root Category has been added");
        println!("{:?}", cat_vec);
    };

    let res_vec = articles::table
        .filter(articles::title.eq("Hello World !"))
        .limit(1)
        .load::<Article>(&conn)
        .expect("Error loading Article");

    if res_vec.len() < 1 {
        insert_root_article();
    } else {
        println!("First Article has been added");
        // println!("{:?}", res_vec);
    };
}