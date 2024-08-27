use diesel::{AsChangeset, ExpressionMethods, Insertable, JoinOnDsl, Queryable, QueryDsl, RunQueryDsl, Selectable, SelectableHelper, SqliteConnection};
use super::schema::{users, posts};
use diesel::associations::HasTable;
use rand;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub login_key: String
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub login_key: &'a str
}


#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub content: &'a str,
    pub user_id: &'a i32,
    pub time: &'a i32,
}


#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Clone)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub time: i32,
    pub user_id: i32
}