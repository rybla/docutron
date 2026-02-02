macro_rules! load_env_var {
    ( $name: ident ) => {
        lazy_static::lazy_static! {
            pub static ref $name: String = std::env::var(stringify!($name)).expect(concat!(
                "This environment variable must be set: ",
                stringify!($name)
            ));
        }
    };
}

load_env_var!(GITHUB_PERSONAL_ACCESS_TOKEN);
load_env_var!(BOOKMARKS_URL_LIST_FILEPATH);
load_env_var!(SAVEDS_URL_LIST_FILEPATH);
load_env_var!(DATABASE_URL);

pub const MIGRATIONS_DIRPATH: &str = "./migrations";

pub const REPOSITORY_URL: &str = "https://github.com/rybla/docutron";
pub const FEEDS_DIRPATH: &str = "site/";
pub const RECENCY_CUTOFF: chrono::Days = chrono::Days::new(30);
pub const MAX_ITEMS_IN_RSS_FEED: usize = 100;
pub const MAX_CHARS_IN_SUMMARY: usize = 600;
