#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_include_static_resources;

use rocket_dyn_templates::Template;
use std::ops::Deref;
use diesel::postfix_operator;
use rocket::http::{Cookie, CookieJar};
use rocket::fs::FileServer;
use rocket::form::{Form, FromForm};
use rocket::http::private::cookie::CookieBuilder;
use rocket::http::private::cookie;
use rocket::response::Redirect;
use crate::context::LoggedInUser;

mod models;
mod schema;
mod context;
mod utils;
mod db;


#[get("/")]
fn index(cookies: &CookieJar) -> Template {
    let mut c = context::Sorts::new("VENTILATION FRONT PAGE; SCREAM OUT!").cookies(cookies);

    // println!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");
    // println!("You are {:?}", c);
    // println!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");

    c.posts_with_username_and_time = Some(db::recent_posts());
    Template::render("index", &c)
}


#[get("/login")]
fn login(cookies: &CookieJar) -> Template {
    let c = context::Sorts::new("Login").cookies(cookies);
    Template::render("login", &c)
}

#[get("/about")]
fn about(cookies: &CookieJar) -> Template
{
    let c = context::Sorts::new("About Ventilator").cookies(cookies);
    Template::render("about", &c)
}

#[get("/register")]
fn get_register(cookies: &CookieJar) -> Template {
    let c = context::Sorts::new("Register").cookies(cookies);
    Template::render("register", &c)
}



// Posting new posts

#[derive(FromForm)]
pub struct PostInput {
    post_content: String, // Post content
}

#[post("/post", data="<user_input>")]
fn post_submit(user_input : Form<PostInput>, cookies: &CookieJar) -> Result<Redirect, Template> {
    let mut c = context::Sorts::new("Post").cookies(cookies);

    let logged_in_user = match c.logged_in_user.clone() {
        Some(user) => {user}
        None => {
            c.error_msg = Some("You need to be logged in (@_@)".to_string());
            return Err(Template::render("index", &c));
        }
    };
    return match db::add_post(&user_input, logged_in_user.id) {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => {
            c.error_msg = Some(e.to_string());
            Err(Template::render("index", &c))
        },
    }
}



#[derive(FromForm)]
pub struct UserRegisterInput {
    username: String, // Username
    password: String, // Password
    password2: String, // Password
}

#[post("/register", data="<user_input>")]
fn post_register(user_input : Form<UserRegisterInput>, cookies: &CookieJar) -> Result<Redirect, Template> {
    let mut c = context::Sorts::new("Register").cookies(cookies);

    match db::register_user(&user_input) {
        Ok(lk) => {return Ok(Redirect::to("/login"));},
        Err(e) => c.error_msg = Some(e.to_string()),
    }

    Err(Template::render("register", &c))
}

#[derive(FromForm)]
pub struct UserLoginInput {
    username: String, // Username
    password: String, // Password
}

#[post("/login", data="<user_input>")]
fn post_login(user_input : Form<UserLoginInput>, cookies: &CookieJar) -> Result<Redirect, Template> {
    let mut c = context::Sorts::new("Login").cookies(cookies);

    match db::login_user(&user_input) {
        Ok(lk) => {
            let set_cookie = CookieBuilder::new("ventilator_login_token", lk.clone())
                .path("/")
                .expires(cookie::time::OffsetDateTime::now_utc() + cookie::time::Duration::days(30)).finish();
            cookies.add(set_cookie);
            c.login_key = Some(lk.to_string());
            return Ok(Redirect::to("/"));
        },
        Err(e) => c.error_msg = Some(e.to_string()),
    }

    Err(Template::render("login", &c))
}

// Make the favicon appear to be served from root
static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(static_resources_initializer!(
            "favicon" => "static/img/favicon.ico"
        ))
        .mount("/", routes![favicon])
        .mount("/", routes![about, index, login, get_register, post_register, post_login, post_submit])

        // Serve the CSS file
        .mount("/static", FileServer::from("static"))
}