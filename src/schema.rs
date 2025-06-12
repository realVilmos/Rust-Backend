// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        name -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}
