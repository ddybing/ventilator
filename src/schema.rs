// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        content -> Text,
        time -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        login_key -> Text,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
