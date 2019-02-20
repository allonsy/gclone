use std::path::PathBuf;
use url;
use super::config;

#[derive(Eq, PartialEq, Debug)]
pub struct ShortHandUrl {
    is_https: bool,
    domain: String,
    path: String
}

#[derive(Eq, PartialEq, Debug)]
pub enum RepoUrl {
    HttpsUrl(url::Url),
    SSHUrl(url::Url),
    ShortUrl(ShortHandUrl)
}

impl RepoUrl {
    pub fn parse(url: &str) -> RepoUrl {
        if is_https_url(url) {
            let parsed_url = url::Url::parse(&url);
            if parsed_url.is_ok() {
                return RepoUrl::HttpsUrl(parsed_url.unwrap());
            } else {
                panic!("Unknown https url: {}", url);
            }
        } else if is_ssh_url(url) {
            let parsed_url = url::Url::parse(&url);
            if parsed_url.is_ok() {
                return RepoUrl::SSHUrl(parsed_url.unwrap());
            } else {
                panic!("Unknown ssh url: {}", url);
            }
        } else {
            let paths: Vec<&str> = url.split("/").collect();
            if paths[0].contains(":") {
                let is_https = false;
                let domain: &str = paths[0].split(":").collect::<Vec<&str>>()[0].split("@").collect::<Vec<&str>>()[1];
                let mut actual_path = paths[0].split(":").collect::<Vec<&str>>()[1].to_string();
                for path in &paths[1..] {
                    actual_path = format!("{}/{}", actual_path, path.to_string());
                }

                RepoUrl::ShortUrl(ShortHandUrl {
                    is_https: is_https,
                    domain: domain.to_string(),
                    path: actual_path
                })
            } else {
                let shorthand = ShortHandUrl {
                    is_https: config::get_is_https(),
                    domain: config::get_domain(),
                    path: url.to_string()
                };
                return RepoUrl::ShortUrl(shorthand);
            }
        }
    }

    pub fn get_clone_url(&self) -> String {
        match self {
            RepoUrl::HttpsUrl(hurl) => {
                hurl.as_str().to_string()
            },
            RepoUrl::SSHUrl(surl) => {
                surl.as_str().to_string()
            },
            RepoUrl::ShortUrl(short) => {
                let url_start = if short.is_https {
                    "https://"
                } else {
                    "git@"
                };
                let div_char = if short.is_https {
                    '/'
                } else {
                    ':'
                };
                format!("{}{}{}{}", url_start, short.domain, div_char, short.path)
            }
        }
    }

    pub fn get_fs_path(&self) -> PathBuf {
        let mut path = config::get_base_path();
        path.push(self.get_domain());
        for p in self.get_sub_path().split("/") {
            path.push(p);
        }

        return path;
    }

    pub fn get_domain(&self) -> String {
        match self {
            RepoUrl::HttpsUrl(hurl) => {
                hurl.domain().unwrap().to_string()
            },
            RepoUrl::SSHUrl(surl) => {
                surl.domain().unwrap().to_string()
            },
            RepoUrl::ShortUrl(short) => {
                short.domain.clone()
            }
        }
    }

    pub fn get_sub_path(&self) -> String {
        match self {
            RepoUrl::HttpsUrl(hurl) => {
                remove_leading_slash(hurl.path())
            },
            RepoUrl::SSHUrl(surl) => {
                remove_leading_slash(surl.path())
            },
            RepoUrl::ShortUrl(short) => {
                short.path.clone()
            }
        }
    }
}

fn remove_leading_slash(path: &str) -> String {
    if &path[0..1] == "/" {
        path[1..].to_string()
    } else {
        path.to_string()
    }
}

fn is_https_url(url: &str) -> bool {
    if url.contains("https://") {
        true
    } else {
        false
    }
}

fn is_ssh_url(url: &str) -> bool {
    if url.contains("ssh://") {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::RepoUrl;
    use super::ShortHandUrl;
    use super::super::config;

    #[test]
    fn test_https_url() {
        let url = "https://github.com/user/repo.git";
        let expected_url = url::Url::parse(url).unwrap();
        assert_eq!(RepoUrl::parse(url), RepoUrl::HttpsUrl(expected_url));
    }

    #[test]
    fn test_https_url_domain() {
        let url = "https://github.com/user/repo.git";
        assert_eq!(RepoUrl::parse(url).get_domain(), "github.com".to_string());
    }

    #[test]
    fn test_https_url_path() {
        let url = "https://github.com/user/repo.git";
        let expected_path = "user/repo.git";
        assert_eq!(RepoUrl::parse(url).get_sub_path(), expected_path);
    }

    #[test]
    fn test_https_url_clone_url() {
        let url = "https://github.com/user/repo.git";
        assert_eq!(RepoUrl::parse(url).get_clone_url(), "https://github.com/user/repo.git");
    }

    #[test]
    fn test_ssh_url() {
        let url = "ssh://aur@aur.archlinux.org/user/repo.git";
        let expected_url = url::Url::parse(url).unwrap();
        assert_eq!(RepoUrl::parse(url), RepoUrl::SSHUrl(expected_url));
    }

    #[test]
    fn test_ssh_url_domain() {
        let url = "ssh://aur@aur.archlinux.org/user/repo.git";
        assert_eq!(RepoUrl::parse(url).get_domain(), "aur.archlinux.org");
    }

    #[test]
    fn test_ssh_url_clone_url() {
        let url = "ssh://aur@aur.archlinux.org/user/repo.git";
        assert_eq!(RepoUrl::parse(url).get_clone_url(), "ssh://aur@aur.archlinux.org/user/repo.git");
    }

    #[test]
    fn test_ssh_url_path() {
        let url = "ssh://aur@aur.archlinux.org/user/repo.git";
        let expected_path = "user/repo.git";
        assert_eq!(RepoUrl::parse(url).get_sub_path(), expected_path);
    }

    #[test]
    fn test_short_ssh_github() {
        let url = "git@github.com:user/repo.git";
        let expected_val = ShortHandUrl {
            is_https: false,
            domain: "github.com".to_string(),
            path: "user/repo.git".to_string()
        };

        assert_eq!(RepoUrl::parse(url), RepoUrl::ShortUrl(expected_val));
    }

    #[test]
    fn test_short_ssh_github_clone_url() {
        let url = "git@github.com:user/repo.git";

        assert_eq!(RepoUrl::parse(url).get_clone_url(), "git@github.com:user/repo.git");
    }

    #[test]
    fn test_simple_url() {
        let url = "user/repo";
        let expected_val = ShortHandUrl {
            is_https: config::get_is_https(),
            domain: config::get_domain(),
            path: "user/repo".to_string()
        };

        assert_eq!(RepoUrl::parse(url), RepoUrl::ShortUrl(expected_val));
    }

    #[test]
    fn test_simple_url_domain() {
        let url = "user/repo";
        assert_eq!(RepoUrl::parse(url).get_domain(), config::get_domain());
    }

    #[test]
    fn test_simple_url_get_clone_url() {
        let url = "user/repo";
        assert_eq!(RepoUrl::parse(url).get_clone_url(), "git@github.com:user/repo");
    }

    #[test]
    fn test_simple_url_get_clone_url_https() {
        let url = "user/repo";
        let mut this_repo = RepoUrl::parse(url);
        match this_repo {
            RepoUrl::ShortUrl(ref mut short) => {
                short.is_https = true;
            },
            _ => {
                assert!(false, "URL should be parsed to shorthand url");
            }
        }
        assert_eq!(this_repo.get_clone_url(), "https://github.com/user/repo");
    }

    #[test]
    fn test_simple_url_path() {
        let url = "user/repo";
        assert_eq!(RepoUrl::parse(url).get_sub_path(), "user/repo");
    }

    #[test]
    fn test_simple_url_multiple() {
        let url = "user/repo/dir1/dir2";
        let expected_val = ShortHandUrl {
            is_https: config::get_is_https(),
            domain: config::get_domain(),
            path: "user/repo/dir1/dir2".to_string()
        };

        assert_eq!(RepoUrl::parse(url), RepoUrl::ShortUrl(expected_val));
    }

    #[test]
    fn test_simple_url_multiple_path() {
        let url = "user/repo/dir1/dir2";
        assert_eq!(RepoUrl::parse(url).get_sub_path(), "user/repo/dir1/dir2");
    }
}