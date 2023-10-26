// @generated automatically by Diesel CLI.

diesel::table! {
    file (id) {
        id -> Text,
        file_source_id -> Nullable<Integer>,
        version_minor -> Nullable<Integer>,
        version_major -> Nullable<Integer>,
        date -> Nullable<Text>,
        has_gps_time -> Nullable<Integer>,
        has_color -> Nullable<Integer>,
        is_compressed -> Nullable<Integer>,
        scale_x -> Nullable<Float>,
        scale_y -> Nullable<Float>,
        scale_z -> Nullable<Float>,
        offset_x -> Nullable<Float>,
        offset_y -> Nullable<Float>,
        offset_z -> Nullable<Float>,
        min_x -> Nullable<Float>,
        min_y -> Nullable<Float>,
        min_z -> Nullable<Float>,
        max_x -> Nullable<Float>,
        max_y -> Nullable<Float>,
        max_z -> Nullable<Float>,
        number_of_points -> Nullable<Integer>,
    }
}

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

diesel::allow_tables_to_appear_in_same_query!(
    file,
    files,
);
