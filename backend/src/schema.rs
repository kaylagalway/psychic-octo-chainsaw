// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (token) {
        token -> Varchar,
        exp_date -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    users (email) {
        displayname -> Varchar,
        email -> Varchar,
        id -> Int4,
        passhash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
