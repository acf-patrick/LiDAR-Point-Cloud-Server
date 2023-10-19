// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        file_id -> Varchar,
        x -> Float8,
        y -> Float8,
        z -> Float8,
        edge -> Float4,
    }
}
