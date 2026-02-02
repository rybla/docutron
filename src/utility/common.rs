use octocrab::Octocrab;
use readability_js::Readability;

pub struct Env {
    pub octocrab: Octocrab,
    pub readability: Readability,
    pub client: reqwest::Client,
}
