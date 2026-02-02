use crate::db::{models::*, queries::*, *};
use anyhow::Result;
use chrono::NaiveDate;

#[test]
fn test_get_doc_tags() -> Result<()> {
    let mut conn = establish_test_connection();

    let doc = NewDocument {
        added_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        url: None,
        source: None,
        title: None,
        published_date: None,
        summary: None,
        fetch_error: None,
        summary_error: None,
    };
    let doc_id = add_document(&mut conn, doc).unwrap().id;
    let tag = NewTag {
        added_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        label: "test".to_string(),
    };
    let tag_id = add_tag(&mut conn, tag).unwrap().id;
    add_document_tags(&mut conn, doc_id, vec![tag_id]).unwrap();
    let doc_tags = get_document_tags(&mut conn, doc_id).unwrap();
    assert_eq!(doc_tags.len(), 1);
    assert_eq!(doc_tags[0].id, tag_id);
    Ok(())
}
