table! {
    articles (id) {
        id -> Uuid,
        user_id -> Uuid,
        category_id -> Uuid,
        release_status -> Int2,
        title -> Varchar,
        content -> Text,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Uuid,
        super_id -> Nullable<Uuid>,
        cat_name -> Varchar,
    }
}

table! {
    comments (id) {
        id -> Uuid,
        article_id -> Uuid,
        nick_name -> Varchar,
        contact_address -> Varchar,
        content -> Text,
        create_time -> Timestamp,
    }
}

table! {
    files (id) {
        id -> Uuid,
        user_id -> Uuid,
        file_path -> Varchar,
    }
}

table! {
    pages (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        content -> Text,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        tag_name -> Varchar,
    }
}

table! {
    tags_with_articles (id) {
        id -> Uuid,
        tag_id -> Uuid,
        article_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        user_email -> Varchar,
        pass_word -> Varchar,
        salt -> Varchar,
        nick_name -> Varchar,
        role_level -> Int2,
        sign_up_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    categories,
    comments,
    files,
    pages,
    tags,
    tags_with_articles,
    users,
);
