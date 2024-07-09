// @generated automatically by Diesel CLI.

diesel::table! {
    tests (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
    }
}
