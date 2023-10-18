// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        path -> Nullable<Varchar>,
    }
}

diesel::table! {
    parts (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        file_id -> Varchar,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        edge -> Nullable<Float4>,
    }
}

diesel::joinable!(parts -> files (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    parts,
);
