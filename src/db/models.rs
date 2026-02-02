use crate::db::schema::{
    author_tags, authors, document_authors, document_tags, documents, tag_group_tags, tag_groups,
    tags,
};
use chrono::NaiveDate;
use diesel::{prelude::*, sqlite};

// ----------------------------------------------------------------------------
// documents
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct Document {
    pub id: i32,
    pub added_date: NaiveDate,
    pub url: String,
    pub bookmark_count: i32,
    pub source: Option<String>,
    pub title: Option<String>,
    pub published_date: Option<String>,
    pub summary: Option<String>,
    pub fetch_error: Option<String>,
    pub summary_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewDocument {
    pub added_date: NaiveDate,
    pub url: String,
    pub bookmark_count: i32,
    pub source: Option<String>,
    pub title: Option<String>,
    pub published_date: Option<String>,
    pub summary: Option<String>,
    pub fetch_error: Option<String>,
    pub summary_error: Option<String>,
}

pub struct NewDocumentBuilder {
    added_date: NaiveDate,
    url: String,
    bookmarked: bool,
}

impl NewDocumentBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            added_date: chrono::Local::now().date_naive(),
            url: url.into(),
            bookmarked: false,
        }
    }

    pub fn added_date(mut self, added_date: NaiveDate) -> Self {
        self.added_date = added_date;
        self
    }

    pub fn bookmarked(mut self, bookmarked: bool) -> Self {
        self.bookmarked = bookmarked;
        self
    }

    pub fn build(self) -> NewDocument {
        NewDocument {
            added_date: self.added_date,
            url: self.url,
            bookmark_count: if self.bookmarked { 1 } else { 0 },
            source: None,
            title: None,
            published_date: None,
            summary: None,
            fetch_error: None,
            summary_error: None,
        }
    }
}

// ----------------------------------------------------------------------------
// authors
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct Author {
    pub id: i32,
    pub added_date: NaiveDate,
    pub url: String,
    pub name: Option<String>,
    pub github_username: Option<String>,
    pub x_username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewAuthor {
    pub added_date: NaiveDate,
    pub url: String,
    pub name: Option<String>,
    pub github_username: Option<String>,
    pub x_username: Option<String>,
}

pub struct NewAuthorBuilder {
    name: Option<String>,
    url: String,
    added_date: NaiveDate,
}

impl NewAuthorBuilder {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            name: None,
            url: url.into(),
            added_date: chrono::Local::now().date_naive(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn build(self) -> NewAuthor {
        NewAuthor {
            added_date: self.added_date,
            url: self.url,
            name: self.name,
            github_username: None,
            x_username: None,
        }
    }
}

// ----------------------------------------------------------------------------
// tags
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct Tag {
    pub id: i32,
    pub added_date: NaiveDate,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewTag {
    pub added_date: NaiveDate,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = tag_groups)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct TagGroup {
    pub id: i32,
    pub added_date: NaiveDate,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = tag_groups)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewTagGroup {
    pub added_date: NaiveDate,
    pub name: Option<String>,
}

// ----------------------------------------------------------------------------
// relations
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = tag_group_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct TagGroupTags {
    pub tag_group_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = tag_group_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewTagGroupTags {
    pub tag_group_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = document_authors)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct DocumentAuthor {
    pub document_id: i32,
    pub author_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = document_authors)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewDocumentAuthor {
    pub document_id: i32,
    pub author_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = document_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct DocumentTag {
    pub document_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = document_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewDocumentTag {
    pub document_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = author_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct AuthorTag {
    pub author_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = author_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewAuthorTag {
    pub author_id: i32,
    pub tag_id: i32,
}
