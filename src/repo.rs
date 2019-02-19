use std::path;
use url;

pub enum RepoUrl {
    Path(path::PathBuf),
    Url(url::Url)
}

impl RepoUrl {
    pub fn parse(url: String) -> RepoUrl {
        RepoUrl::Path(path::PathBuf::new())
    }
}