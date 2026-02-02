# CHANGELOG 2026-02-02-17-16-48 Implement duplicate document handling in add_document and fix test instantiation.

Updated the add_document function in src/db/queries.rs to handle duplicate URLs by checking for existing records; if a duplicate is found and the new entry is bookmarked, the existing document's bookmark_count is incremented, otherwise the existing document is returned. Additionally, updated the NewDocument instantiation in src/db/test.rs to include the required url and bookmark_count fields.

