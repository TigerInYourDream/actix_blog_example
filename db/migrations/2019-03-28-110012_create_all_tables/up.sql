-- Your SQL goes here

create extension IF NOT EXISTS pgcrypto;

CREATE TABLE users
(
  id           uuid PRIMARY KEY        default gen_random_uuid(),
  user_email   varchar unique NOT NULL,
  pass_word    varchar        NOT NULL,
  salt         varchar        NOT NULL,
  nick_name    varchar        NOT NULL,
  role_level   smallint       NOT NULL default 100,
  sign_up_time timestamp      NOT NULL default current_timestamp
);

CREATE TABLE categories
(
  id       uuid PRIMARY KEY default gen_random_uuid(),
  super_id uuid,
  cat_name varchar NOT NULL
  -- cat_root boolean NOT NULL default false
);

CREATE TABLE articles
(
  id             uuid PRIMARY KEY   default gen_random_uuid(),
  user_id        uuid      NOT NULL,
  category_id    uuid      NOT NULL,
  release_status smallint  NOT NULL default 0,
  title          varchar   NOT NULL,
  content        text      NOT NULL,
  create_time    timestamp NOT NULL default current_timestamp,
  update_time    timestamp NOT NULL default current_timestamp
);

CREATE TABLE pages
(
  id      uuid PRIMARY KEY default gen_random_uuid(),
  user_id uuid    NOT NULL,
  title   varchar NOT NULL,
  content text    NOT NULL
);

CREATE TABLE tags
(
  id       uuid PRIMARY KEY default gen_random_uuid(),
  tag_name varchar unique NOT NULL
);

CREATE TABLE tags_With_articles
(
  id         uuid PRIMARY KEY default gen_random_uuid(),
  tag_id     uuid NOT NULL,
  article_id uuid NOT NULL
);

CREATE TABLE files
(
  id        uuid PRIMARY KEY default gen_random_uuid(),
  user_id   uuid    NOT NULL,
  file_path varchar NOT NULL
);

CREATE TABLE comments
(
  id              uuid PRIMARY KEY   default gen_random_uuid(),
  article_id      uuid      NOT NULL,
  nick_name       varchar   NOT NULL,
  contact_address varchar   NOT NULL,
  content         text      NOT NULL,
  create_time     timestamp NOT NULL default current_timestamp
);