// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Text,
        file_source_id -> Integer,
        version_minor -> Integer,
        version_major -> Integer,
        date -> Nullable<Text>,
        has_gps_time -> Integer,
        has_color -> Integer,
        is_compressed -> Integer,
        scale_x -> Float,
        scale_y -> Float,
        scale_z -> Float,
        offset_x -> Float,
        offset_y -> Float,
        offset_z -> Float,
        min_x -> Float,
        min_y -> Float,
        min_z -> Float,
        max_x -> Float,
        max_y -> Float,
        max_z -> Float,
        number_of_points -> BigInt,
    }
}

diesel::table! {
    parts (id) {
        id -> Text,
        file_id -> Text,
        x -> Float,
        y -> Float,
        z -> Float,
        edge -> Float,
    }
}

diesel::joinable!(parts -> files (file_id));

diesel::allow_tables_to_appear_in_same_query!(
    files,
    parts,
);
