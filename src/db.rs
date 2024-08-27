use std::env;
use diesel::{Connection, ExpressionMethods, IntoSql, JoinOnDsl, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper, SqliteConnection};
use dotenv::dotenv;
use crate::{schema, UserLoginInput, utils, PostInput};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::http::Method::Post;
use crate::context::{LoggedInUser, PostWithUsername, PostWithUsernameAndTime};

use crate::models::{NewPost, NewUser, User};
use crate::schema::posts::dsl::posts;
use crate::schema::users::dsl::users;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn get_all_recent_posts(conn: &mut SqliteConnection) -> Vec<PostWithUsername> {
    use crate::schema::{posts, users};

    let results: Vec<PostWithUsername> = posts
        .inner_join(users.on(posts::user_id.eq(&users::id)))
        .select((posts::id, posts::content, posts::time, posts::user_id, users::username))
        .order(posts::time.desc())
        .load(conn)
        .expect("Error loading posts with usernames");

    results
}

pub fn recent_posts() -> Vec<PostWithUsernameAndTime> {
    let conn = &mut establish_connection();

    let posts_with_username = get_all_recent_posts(conn);
    let mut posts_with_username_and_time : Vec<PostWithUsernameAndTime> = vec!();
    posts_with_username.iter().for_each(|p| {
        posts_with_username_and_time.push(PostWithUsernameAndTime::new(p.clone()))
    });
    posts_with_username_and_time
}

pub fn register_user(user_input: &super::UserRegisterInput) -> Result<(), Box<dyn Error>> {
    let conn = &mut establish_connection();

    // Check if the password and password confirmation are the same.
    // If not, you shall not pass - Gandalf.
    if &user_input.password.to_string() != &user_input.password2.to_string() { return Err(Box::try_from("Oh no ;_; Mismatching password (-_-;)").unwrap()) }

    let new_user = NewUser {
        username: &user_input.username.to_string(),
        password: &user_input.password.to_string(),
        login_key: &*utils::random::gen_random_string(128)
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)?;

    Ok(())
}

pub fn login_user(user_input: &UserLoginInput) -> Result<String, Box<dyn Error>> {
    let conn = &mut establish_connection();


    let something = users
        .filter(schema::users::username.eq(&user_input.username))
        .filter(schema::users::password.eq(&user_input.password))
        .select(User::as_select())
        .load(conn)?;

    if something.len() != 1 {
        return Err(Box::try_from("Oh no ;_; Not found (ｕ_ｕ*)").unwrap())
    }

    Ok(something[0].login_key.clone())
}

pub fn get_user(key: &String) -> Option<LoggedInUser> {
    let conn = &mut establish_connection();

    let result = users
        .filter(schema::users::login_key.eq(&key))
        .select(User::as_select())
        .load(conn);

    match result {
        Ok(u) if u.len() != 1 => None,
        Ok(u) => Some(LoggedInUser {
            name: u[0].username.to_string(),
            id: u[0].id
        }),
        Err(_) => None,
    }
}

pub fn add_post(post_content: &PostInput, user_id: i32) -> Result<(), Box<dyn Error>>
{
    let conn = &mut establish_connection();

    let now = SystemTime::now();
    let time = now.duration_since(UNIX_EPOCH)?.as_secs() as i32;

    let new_post = NewPost
    {
        content : &*post_content.post_content,
        user_id: &user_id,
        time: &time
    };

    diesel::insert_into(posts).values(new_post).execute(conn)?;

    Ok(())
}