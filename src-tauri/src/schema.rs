// @generated automatically by Diesel CLI.

diesel::table! {
    students (register) {
        register -> Integer,
        fullname -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        status -> Text,
        semester -> Text,
        group -> Text,
        turn -> Text,
        level -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
