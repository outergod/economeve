// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    tokens (character_id) {
        character_id -> Integer,
        access_token -> Text,
        refresh_token -> Text,
        expiry -> Timestamp,
    }
}

diesel::joinable!(tokens -> characters (character_id));

diesel::allow_tables_to_appear_in_same_query!(characters, tokens,);
