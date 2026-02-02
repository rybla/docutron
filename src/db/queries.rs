use crate::db::{
    models::*,
    schema::{doc_tags, docs, tags},
};
use chrono::NaiveDate;
use diesel::{insert_into, prelude::*};

/// Insert a new document into the database.
pub fn insert_doc(conn: &mut SqliteConnection, doc: NewDoc) -> QueryResult<Doc> {
    insert_into(docs::dsl::docs).values(&doc).get_result(conn)
}

/// Retrieves a document from the database by its ID.
pub fn get_doc(conn: &mut SqliteConnection, doc_id: i32) -> QueryResult<Doc> {
    docs::dsl::docs.find(doc_id).first(conn)
}

/// Retrieves all documents from the database in order of decreasing `added_date`.
pub fn get_docs(conn: &mut SqliteConnection) -> QueryResult<Vec<Doc>> {
    docs::dsl::docs
        .order_by(docs::dsl::added_date.desc())
        .load(conn)
}

/// Retrieves all documents from the database that have an `added_date` after the given date, in order of decreasing `added_date`.
pub fn get_docs_after(conn: &mut SqliteConnection, date: NaiveDate) -> QueryResult<Vec<Doc>> {
    docs::dsl::docs
        .filter(docs::dsl::added_date.gt(date))
        .order_by(docs::dsl::added_date.desc())
        .load(conn)
}

/// Retrieves at most `limit` documents from the database in order of decreasing `added_date`.
pub fn get_docs_limit(_conn: &mut SqliteConnection, _limit: usize) -> QueryResult<Vec<Doc>> {
    unimplemented!()
}

/// Retrieves all documents from the database that have a given tag, in order of decreasing `added_date`.
pub fn get_docs_by_tag(_conn: &mut SqliteConnection, _tag_id: i32) -> QueryResult<Vec<Doc>> {
    unimplemented!()
}

/// Insert a new tag into the database.
pub fn insert_tag(conn: &mut SqliteConnection, tag: NewTag) -> QueryResult<Tag> {
    insert_into(tags::dsl::tags).values(&tag).get_result(conn)
}

/// Inserts new tags into the database.
pub fn insert_tags(conn: &mut SqliteConnection, tags: Vec<NewTag>) -> QueryResult<Vec<Tag>> {
    insert_into(tags::dsl::tags).values(&tags).get_results(conn)
}

/// Retrieves a tag from the database by its ID.
pub fn get_tag(conn: &mut SqliteConnection, tag_id: i32) -> QueryResult<Tag> {
    tags::dsl::tags.find(tag_id).first(conn)
}

/// Adds relations between a document and its tags into the database.
pub fn insert_doc_tags(
    conn: &mut SqliteConnection,
    doc_id: i32,
    tag_ids: Vec<i32>,
) -> QueryResult<()> {
    let new_doc_tags: Vec<NewDocTag> = tag_ids
        .into_iter()
        .map(|tag_id| NewDocTag { doc_id, tag_id })
        .collect();

    insert_into(doc_tags::dsl::doc_tags)
        .values(&new_doc_tags)
        .execute(conn)
        .map(|_| ())
}

/// Retrieves all tags that are related to a given document ID from the database.
pub fn get_doc_tags(conn: &mut SqliteConnection, doc_id: i32) -> QueryResult<Vec<Tag>> {
    doc_tags::dsl::doc_tags
        .inner_join(tags::dsl::tags)
        .filter(doc_tags::dsl::doc_id.eq(doc_id))
        .select(Tag::as_select())
        .load(conn)
}
