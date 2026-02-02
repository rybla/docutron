use crate::db::schema::{
    author_tags, authors, document_tags, documents, tag_group_tags, tag_groups, tags,
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
    pub bookmark_count: i32,
    pub url: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub published_date: Option<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewDocument {
    pub added_date: NaiveDate,
    pub url: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub published_date: Option<String>,
    pub summary: Option<String>,
}

pub struct NewDocumentBuilder {
    added_date: NaiveDate,
    url: Option<String>,
    source: Option<String>,
    title: Option<String>,
    published_date: Option<String>,
    summary: Option<String>,
}

impl NewDocumentBuilder {
    pub fn new() -> Self {
        Self {
            added_date: chrono::Local::now().date_naive(),
            url: None,
            source: None,
            title: None,
            published_date: None,
            summary: None,
        }
    }

    pub fn added_date(mut self, added_date: NaiveDate) -> Self {
        self.added_date = added_date;
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn published_date(mut self, published_date: impl Into<String>) -> Self {
        self.published_date = Some(published_date.into());
        self
    }

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn build(self) -> NewDocument {
        NewDocument {
            added_date: self.added_date,
            url: self.url,
            source: self.source,
            title: self.title,
            published_date: self.published_date,
            summary: self.summary,
        }
    }
}

impl Default for NewDocumentBuilder {
    fn default() -> Self {
        Self::new()
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
    pub name: Option<String>,
    pub website_url: Option<String>,
    pub github_username: Option<String>,
    pub x_username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewAuthor {
    pub name: Option<String>,
    pub website_url: Option<String>,
    pub github_username: Option<String>,
    pub x_username: Option<String>,
}

pub struct NewAuthorBuilder {
    name: Option<String>,
    website_url: Option<String>,
    github_username: Option<String>,
    x_username: Option<String>,
}

impl NewAuthorBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            website_url: None,
            github_username: None,
            x_username: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn website_url(mut self, website_url: impl Into<String>) -> Self {
        self.website_url = Some(website_url.into());
        self
    }

    pub fn github_username(mut self, github_username: impl Into<String>) -> Self {
        self.github_username = Some(github_username.into());
        self
    }

    pub fn x_username(mut self, x_username: impl Into<String>) -> Self {
        self.x_username = Some(x_username.into());
        self
    }

    pub fn build(self) -> NewAuthor {
        NewAuthor {
            name: self.name,
            website_url: self.website_url,
            github_username: self.github_username,
            x_username: self.x_username,
        }
    }
}

impl Default for NewAuthorBuilder {
    fn default() -> Self {
        Self::new()
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
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewTag {
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = tag_groups)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct TagGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = tag_groups)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewTagGroup {
    pub name: String,
}

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
