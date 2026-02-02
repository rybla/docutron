# CHANGELOG 2026-02-02-14-24-38 Use explicit MigrationHarness for test migrations.

Explictly annotate `conn` as a `MigrationHarness` instance in `establish_test_connection` by calling `MigrationHarness::run_pending_migrations(&mut conn, MIGRATIONS)`. This clears up ambiguity and ensures strict trait usage. Also moved the `diesel_migrations::MigrationHarness` import behind `#[cfg(test)]` to fix an unused import warning in non-test builds.

