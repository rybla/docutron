# CHANGELOG 2026-02-02-15-38-25 Implement `clean_url` function to sanitize URLs by removing queries and fragments.

Implemented the `clean_url` function in `src/utility/url_list.rs` using the `url` crate. This function now correctly parses URLs and removes query parameters and fragments, returning a cleaned URL string. If parsing fails, it logs a warning and returns the original URL.

