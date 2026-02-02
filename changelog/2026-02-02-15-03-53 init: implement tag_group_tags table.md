# CHANGELOG 2026-02-02-15-03-53 init: implement tag_group_tags table

Implemented the `tag_group_tags` table in `migrations/2026-02-02-172523-0000_init/up.sql` with a composite primary key and foreign key constraints to `tag_groups` and `tags` (with cascade delete), completing the schema definition for relating tag groups to tags.

