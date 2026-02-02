// @generated automatically by Diesel CLI.

diesel::table! {
    author_tags (author_id, tag_id) {
        author_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    authors (id) {
        id -> Integer,
        url -> Text,
        added_date -> Date,
        name -> Nullable<Text>,
        github_username -> Nullable<Text>,
        x_username -> Nullable<Text>,
    }
}

diesel::table! {
    document_authors (document_id, author_id) {
        document_id -> Integer,
        author_id -> Integer,
    }
}

diesel::table! {
    document_tags (document_id, tag_id) {
        document_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    documents (id) {
        id -> Integer,
        added_date -> Date,
        url -> Text,
        bookmark_count -> Integer,
        source -> Nullable<Text>,
        title -> Nullable<Text>,
        published_date -> Nullable<Text>,
        summary -> Nullable<Text>,
        fetch_error -> Nullable<Text>,
        summary_error -> Nullable<Text>,
    }
}

diesel::table! {
    tag_group_tags (tag_group_id, tag_id) {
        tag_group_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tag_groups (id) {
        id -> Integer,
        added_date -> Date,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        added_date -> Date,
        label -> Text,
    }
}

diesel::joinable!(author_tags -> authors (author_id));
diesel::joinable!(author_tags -> tags (tag_id));
diesel::joinable!(document_authors -> authors (author_id));
diesel::joinable!(document_authors -> documents (document_id));
diesel::joinable!(document_tags -> documents (document_id));
diesel::joinable!(document_tags -> tags (tag_id));
diesel::joinable!(tag_group_tags -> tag_groups (tag_group_id));
diesel::joinable!(tag_group_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    author_tags,
    authors,
    document_authors,
    document_tags,
    documents,
    tag_group_tags,
    tag_groups,
    tags,
);
