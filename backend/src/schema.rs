// @generated automatically by Diesel CLI.

diesel::table! {
    users (email) {
        displayname -> Varchar,
        email -> Varchar,
        id -> Int4,
        passhash -> Varchar,
    }
}
