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
