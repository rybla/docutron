# CHANGELOG 2026-02-02-15-29-52 Implement bookmark and saved URL list file readers.

Implemented the `read_bookmarks_url_list` and `read_saved_url_list` functions in `src/utility/url_list.rs` to read newline-separated lists of URLs from file paths specified in the configuration. The implementation uses `std::fs::read_to_string` to read the files and processes the content by trimming lines and filtering out empty ones, returning a `Result<Vec<String>>` with appropriate error context.

