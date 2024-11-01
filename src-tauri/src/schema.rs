// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        subject -> Text,
        context -> Text,
    }
}
