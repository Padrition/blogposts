// @generated automatically by Diesel CLI.

diesel::table! {
    blogpost (id) {
        id -> Int4,
        text -> Varchar,
        publication_date -> Date,
        post_image_path -> Nullable<Varchar>,
        username -> Varchar,
        avatar_path -> Nullable<Varchar>,
    }
}
