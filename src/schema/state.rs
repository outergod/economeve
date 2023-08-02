// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Text,
        refresh_token -> Text,
    }
}
