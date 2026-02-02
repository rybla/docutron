---
trigger: always_on
---

Before you submit a change, you must perform the following steps in order:

1. **Type checking**. Run the command `cargo check` to run the type checker. If there are any warnings or errors, address ALL these diagnostics and then run the type checker again.
2. **Building**. Run the command `cargo build` to build the project. If there are any warnings or errors, address ALL these diagnostics and then the build again.
3. **Testing**. Run the command `cargo test` to run the test suite. If there are any failures, diagnose those failures and then determine fixes.
4. **Changelog**. Run the command `cargo --bin add_changelog_entry` to initiate the client for adding a new changelog entry. Follow its instructions.

Once you have completed all of the above steps successfully, report back to the user.