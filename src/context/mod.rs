use diesel::Queryable;
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use rocket::http::{Cookie, CookieJar};
use crate::db;
use crate::schema::users::login_key;

// This is a post with a username
#[derive(Serialize, Queryable, Clone, Debug)]
pub struct PostWithUsername
{
    pub id: i32,
    pub content: String,
    pub time: i32,
    pub user_id: i32,
    pub username: String
}

// This is a post with both username AND time
#[derive(Serialize, Queryable, Clone, Debug)]
pub struct PostWithUsernameAndTime
{
    pub post: PostWithUsername,
    pub time: String
}
impl PostWithUsernameAndTime {
    pub fn new(post: PostWithUsername) -> PostWithUsernameAndTime {
        let naive = NaiveDateTime::from_timestamp(post.time as i64, 0);

        let offset = FixedOffset::east(2 * 3600); // 2 hours in seconds
        let datetime = offset.from_utc_datetime(&naive);

        Self {
            post,
            time: datetime.format("%Y-%m-%d %H:%M:%S").to_string() // note the corrected format string
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LoggedInUser {
    pub name: String,
    pub id: i32
}


#[derive(Serialize, Debug, Clone)]
pub struct Sorts<'a> {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts_with_username_and_time: Option<Vec<PostWithUsernameAndTime>>,
    pub error_msg: Option<String>,
    pub login_key: Option<String>,
    #[serde(skip_serializing)]
    pub cookies: Option<CookieJar<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logged_in_user: Option<LoggedInUser>
}

impl<'a> Sorts<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            posts_with_username_and_time: None,
            error_msg: None,
            login_key: None,
            cookies: None,
            logged_in_user: None,
        }
    }

    pub fn cookies(mut self, cookies: &'a CookieJar) -> Self {
        // Check if logged in
        self.logged_in_user = cookies
            .get("ventilator_login_token")
            .and_then(|key| db::get_user(&key.value().to_string()));

        // self.logged_in_user = super::db::get_user()
        self.cookies = Some(cookies.clone());
        self
    }
}