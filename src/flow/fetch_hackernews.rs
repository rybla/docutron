use anyhow::Result;

lazy_static::lazy_static! {
    static ref KEYWORDS: Vec<String> = [
        "programming languages",
        "type theory",
        "type system",
        "haskell",
        "AI",
        "developer tools",
        "video game development",
        "functional programming",
        "dev tools",
        "rust",
        "purescript",
        "compilers",
        "developer experience",
        "category theory",
        "liquid haskell",
        "monad",
        "metaprogramming",
        "mac mini",
        "logic programming",
        "effect systems for purely functional programming langauges",
        "typescript",
        "ocaml",
        "rust",
        "purescript",
        "compiler",
        "mcp",
        "prediction market",
        "homotopy"
        ].into_iter().map(|s| s.to_owned()).collect();
    static ref TOPICS: Vec<String> = [
        "haskell",
        "functional",
        "google",
        "software",
        "korea",
        "japan",
        "singapore",
        "palantir",
        "math",
        "meta",
        "gwern",
        "type",
        "lang",
        "syntax",
        "semantics",
        "github"
        ].into_iter().map(|s| s.to_owned()).collect();
}

/// Fetch the top stories from Hacker News, then insert those urls into the `documents` table in the database.
///
/// If an insertion fails, a warning is logged and the process continues.
pub async fn fetch_hackernews() -> Result<()> {
    unimplemented!()
}
