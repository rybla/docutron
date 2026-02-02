use crate::db::{
    models::*,
    schema::{document_tags, documents, tags},
};
use chrono::NaiveDate;
use diesel::{insert_into, prelude::*};

/// Insert a new document into the database.
pub fn add_document(conn: &mut SqliteConnection, document: NewDocument) -> QueryResult<Document> {
    use crate::db::schema::documents::dsl::*;

    let opt_existing_document = documents
        .filter(url.eq(&document.url))
        .first::<Document>(conn)
        .optional()?;

    if let Some(existing_document) = opt_existing_document {
        if document.bookmark_count > 0 {
            diesel::update(documents.find(existing_document.id))
                .set(bookmark_count.eq(bookmark_count + 1))
                .get_result(conn)
        } else {
            Ok(existing_document)
        }
    } else {
        insert_into(documents).values(&document).get_result(conn)
    }
}

/// Retrieves a document from the database by its ID.
pub fn get_document(conn: &mut SqliteConnection, document_id: i32) -> QueryResult<Document> {
    documents::dsl::documents.find(document_id).first(conn)
}

/// Retrieves all documents from the database in order of decreasing `added_date`.
pub fn get_documents(conn: &mut SqliteConnection) -> QueryResult<Vec<Document>> {
    documents::dsl::documents
        .order_by(documents::dsl::added_date.desc())
        .load(conn)
}

/// Retrieves all documents from the database that have an `added_date` after the given date, in order of decreasing `added_date`.
pub fn get_documents_after(
    conn: &mut SqliteConnection,
    date: NaiveDate,
) -> QueryResult<Vec<Document>> {
    documents::dsl::documents
        .filter(documents::dsl::added_date.gt(date))
        .order_by(documents::dsl::added_date.desc())
        .load(conn)
}

/// Retrieves at most `limit` documents from the database in order of decreasing `added_date`.
pub fn get_documents_limit(
    _conn: &mut SqliteConnection,
    _limit: usize,
) -> QueryResult<Vec<Document>> {
    unimplemented!()
}

/// Retrieves all documents from the database that have a given tag, in order of decreasing `added_date`.
pub fn get_documents_by_tag(
    _conn: &mut SqliteConnection,
    _tag_id: i32,
) -> QueryResult<Vec<Document>> {
    unimplemented!()
}

/// Insert a new tag into the database.
pub fn add_tag(conn: &mut SqliteConnection, tag: NewTag) -> QueryResult<Tag> {
    insert_into(tags::dsl::tags).values(&tag).get_result(conn)
}

/// Inserts new tags into the database.
pub fn add_tags(conn: &mut SqliteConnection, tags: Vec<NewTag>) -> QueryResult<Vec<Tag>> {
    insert_into(tags::dsl::tags).values(&tags).get_results(conn)
}

/// Retrieves a tag from the database by its ID.
pub fn get_tag(conn: &mut SqliteConnection, tag_id: i32) -> QueryResult<Tag> {
    tags::dsl::tags.find(tag_id).first(conn)
}

/// Adds relations between a document and its tags into the database.
pub fn add_document_tags(
    conn: &mut SqliteConnection,
    document_id: i32,
    tag_ids: Vec<i32>,
) -> QueryResult<()> {
    let new_document_tags: Vec<NewDocumentTag> = tag_ids
        .into_iter()
        .map(|tag_id| NewDocumentTag {
            document_id,
            tag_id,
        })
        .collect();

    insert_into(document_tags::dsl::document_tags)
        .values(&new_document_tags)
        .execute(conn)
        .map(|_| ())
}

/// Retrieves all tags that are related to a given document ID from the database.
pub fn get_document_tags(conn: &mut SqliteConnection, document_id: i32) -> QueryResult<Vec<Tag>> {
    document_tags::dsl::document_tags
        .inner_join(tags::dsl::tags)
        .filter(document_tags::dsl::document_id.eq(document_id))
        .select(Tag::as_select())
        .load(conn)
}
