// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Text,
        file_id -> Text,
        x -> Float,
        y -> Float,
        z -> Float,
        edge -> Float,
    }
}
