// @generated automatically by Diesel CLI.

diesel::table! {
    doc_tags (doc_id, tag_id) {
        doc_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    docs (id) {
        id -> Integer,
        added_date -> Date,
        url -> Nullable<Text>,
        source -> Nullable<Text>,
        title -> Nullable<Text>,
        published_date -> Nullable<Text>,
        summary -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        label -> Text,
    }
}

diesel::joinable!(doc_tags -> docs (doc_id));
diesel::joinable!(doc_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(doc_tags, docs, tags,);
