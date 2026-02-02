# CHANGELOG 2026-02-02-13-41-23 Implement `get_doc_tags` database query

Implemented `get_doc_tags` in `src/db/queries.rs`. This function retrieves all tags associated with a given `doc_id` by performing an inner join between `doc_tags` and `tags` tables and filtering by the document ID.

