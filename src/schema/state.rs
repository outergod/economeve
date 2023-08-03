// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Integer,
        name -> Text,
        access_token -> Text,
        refresh_token -> Text,
        expiry -> Timestamp,
    }
}
