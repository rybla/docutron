use crate::db::schema::{doc_tags, docs, tags};
use chrono::NaiveDate;
use diesel::{prelude::*, sqlite};

#[derive(Debug, Clone, PartialEq, Eq, Queryable, Selectable)]
#[diesel(table_name = docs)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct Doc {
    pub id: i32,
    pub added_date: NaiveDate,
    pub url: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub published_date: Option<String>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = docs)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewDoc {
    pub added_date: NaiveDate,
    pub url: Option<String>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub published_date: Option<String>,
    pub summary: Option<String>,
}

pub struct NewDocBuilder {
    added_date: NaiveDate,
    url: Option<String>,
    source: Option<String>,
    title: Option<String>,
    published_date: Option<String>,
    summary: Option<String>,
}

impl NewDocBuilder {
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

    pub fn build(self) -> NewDoc {
        NewDoc {
            added_date: self.added_date,
            url: self.url,
            source: self.source,
            title: self.title,
            published_date: self.published_date,
            summary: self.summary,
        }
    }
}

impl Default for NewDocBuilder {
    fn default() -> Self {
        Self::new()
    }
}

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
#[diesel(table_name = doc_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct DocTag {
    pub doc_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Insertable)]
#[diesel(table_name = doc_tags)]
#[diesel(check_for_backend(sqlite::Sqlite))]
pub struct NewDocTag {
    pub doc_id: i32,
    pub tag_id: i32,
}
