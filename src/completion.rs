use crate::config;
use glob::glob;
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;

pub fn get_matches(config: &config::Config, input: &str) -> Vec<(String, String)> {
    let mut matches = get_unsorted_matches(config, input);
    matches.sort_by(hint_sorter);
    matches
}

fn get_unsorted_matches(conf: &config::Config, input: &str) -> Vec<(String, String)> {
    if input.contains(':') {
        let broken_input: Vec<&str> = input.split(':').collect();
        let domain = broken_input[0];
        let path = broken_input[1];
        return scan_single_tld(&conf, domain, path, true, true);
    }

    if input.contains('/') {
        let domain = conf.get_domain();
        return scan_single_tld(&conf, domain, input, true, false);
    }

    if input == "" {
        return get_top_level_hints(&conf);
    }

    search_for_component(&conf, input)
}

fn get_all_non_default_tlds(conf: &config::Config) -> Vec<String> {
    let default_tld = conf.get_domain();
    let tlds = get_all_tlds(conf);
    let mut new_tlds = Vec::new();
    for tld in tlds {
        if &tld != default_tld {
            new_tlds.push(tld);
        }
    }

    new_tlds
}

fn get_all_tlds(conf: &config::Config) -> Vec<String> {
    let base_path = conf.get_base_path();
    let mut tlds = Vec::new();

    let cur_dir_res = fs::read_dir(base_path);
    if cur_dir_res.is_err() {
        return tlds;
    }
    let base_dir = cur_dir_res.unwrap();
    for dir in base_dir {
        if dir.is_ok() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                tlds.push(
                    dir_path
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }
        }
    }

    tlds
}

fn scan_single_tld(
    conf: &config::Config,
    tld: &str,
    path: &str,
    add_short_desc: bool,
    add_tld_prefix: bool,
) -> Vec<(String, String)> {
    let mut completions = Vec::new();
    let base_path = conf.get_base_path();
    let full_path = format!("{}/{}/{}*", base_path.to_str().unwrap(), tld, path);
    let path_prefix = PathBuf::from(format!("{}/{}", base_path.to_str().unwrap(), tld));
    let glob_results = glob(&full_path);
    if glob_results.is_err() {
        eprintln!("invalid glob pattern");
        return completions;
    }
    for entry in glob_results.unwrap().filter_map(Result::ok) {
        let glob_path = entry;
        if glob_path.is_dir() {
            let relative_path = glob_path.strip_prefix(path_prefix.clone());
            if relative_path.is_ok() {
                let rel_path = relative_path.unwrap();
                let input_name = if add_tld_prefix {
                    format!("{}:{}/", tld, rel_path.display())
                } else {
                    format!("{}/", rel_path.display())
                };
                let short_name = if add_short_desc {
                    format!("{}/", glob_path.file_name().unwrap().to_str().unwrap())
                } else {
                    input_name.clone()
                };
                completions.push((input_name, short_name));
            }
        }
    }

    completions
}

fn get_top_level_hints(conf: &config::Config) -> Vec<(String, String)> {
    let mut hints = Vec::new();

    let non_default_tlds = get_all_tlds(conf);
    for domain in non_default_tlds {
        let domain_colon = format!("{}:", domain);
        hints.push((domain_colon.clone(), domain_colon));
    }

    let default_domain = conf.get_domain();
    let base_path = conf.get_base_path();
    let search_path = base_path.join(default_domain);

    let search_dir_res = fs::read_dir(search_path);
    if search_dir_res.is_err() {
        return hints;
    }

    let search_dir = search_dir_res.unwrap();
    for dir in search_dir {
        if dir.is_ok() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                let file_name = format!("{}/", dir_path.file_name().to_str().unwrap());
                hints.push((file_name.clone(), file_name));
            }
        }
    }

    hints
}

fn search_for_component(conf: &config::Config, input: &str) -> Vec<(String, String)> {
    let mut hints = Vec::new();
    let default_domain = conf.get_domain();
    let max_depth = conf.get_default_search_depth();

    let base_path = conf.get_base_path();
    let default_path = format!("{}", base_path.join(default_domain).display());
    let default_hints = list_components(&default_path, "", input, 0, max_depth);

    if default_domain.starts_with(input) {
        hints.push((
            format!("{}:", default_domain),
            format!("{}:", default_domain),
        ));
    }

    for (hint, short) in default_hints {
        hints.push((hint.clone(), short.clone()));
    }

    for tld in get_all_non_default_tlds(conf) {
        if tld.starts_with(input) {
            hints.push((format!("{}:", tld), format!("{}:", tld)));
        }
        let search_path = format!("{}/{}", base_path.display(), tld);
        for hint in list_components(&search_path, &format!("{}:", tld), input, 0, max_depth) {
            hints.push(hint);
        }
    }

    hints
}

fn list_components(
    path: &str,
    prefix: &str,
    input: &str,
    depth: usize,
    max_depth: usize,
) -> Vec<(String, String)> {
    if depth >= max_depth {
        return Vec::new();
    }

    let mut hints = Vec::new();

    let search_dir_res = fs::read_dir(path);
    if search_dir_res.is_err() {
        return hints;
    }

    let search_dir = search_dir_res.unwrap();
    for dir in search_dir {
        if dir.is_ok() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                let file_path = dir_path.path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with(input) {
                    let hint_name = format!("{}{}/", prefix, file_name);
                    hints.push((hint_name.clone(), hint_name.clone()));
                }
                let sub_path = format!("{}/{}", path, file_name);
                let sub_hints = list_components(
                    &sub_path,
                    &format!("{}{}/", prefix, file_name),
                    input,
                    depth + 1,
                    max_depth,
                );
                for sub_hint in sub_hints {
                    hints.push(sub_hint);
                }
            }
        }
    }

    hints
}

pub fn hint_sorter(tup1: &(String, String), tup2: &(String, String)) -> Ordering {
    let desc1 = &tup1.1;
    let desc2 = &tup2.1;

    if desc1.to_lowercase() < desc2.to_lowercase() {
        return Ordering::Less;
    }
    if desc2.to_lowercase() < desc1.to_lowercase() {
        return Ordering::Greater;
    }

    Ordering::Equal
}

#[cfg(test)]
mod test {
    use super::get_matches;
    use crate::config;

    fn get_testing_config() -> config::Config {
        let mut conf = config::get_config();
        let mut base_path = std::env::current_dir().unwrap();
        base_path.push("test");
        base_path.push("completions");
        conf.set_base_path(base_path);
        conf.set_default_domain("github.com".to_string());
        conf
    }

    fn conv_matches(expected_mathes: Vec<(&str, &str)>) -> Vec<(String, String)> {
        let mut new_matches = Vec::new();
        for (a, b) in expected_mathes {
            new_matches.push((a.to_string(), b.to_string()));
        }

        new_matches
    }

    #[test]
    fn test_no_input() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "");
        let expected_matches = vec![
            ("allonsy/", "allonsy/"),
            ("aur.archlinux.org:", "aur.archlinux.org:"),
            ("github.com:", "github.com:"),
            ("gitlab.com:", "gitlab.com:"),
        ];
        assert_eq!(matches, conv_matches(expected_matches));
    }

    #[test]
    fn test_short_path() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "allonsy/gclone/");
        let expected_matches = vec![
            ("allonsy/gclone/src/", "src/"),
            ("allonsy/gclone/test/", "test/"),
        ];
        assert_eq!(matches, conv_matches(expected_matches));
    }

    #[test]
    fn test_short_path_incomplete() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "allonsy/gclone/src/co");
        let expected_matches = vec![("allonsy/gclone/src/code/", "code/")];
        assert_eq!(matches, conv_matches(expected_matches));
    }

    #[test]
    fn test_full_path() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "gitlab.com:allonsy/");
        let expected_matches = vec![("gitlab.com:allonsy/repo/", "repo/")];
        assert_eq!(matches, conv_matches(expected_matches));
    }

    #[test]
    fn test_full_path_incomplete() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "gitlab.com:allonsy/re");
        let expected_matches = vec![("gitlab.com:allonsy/repo/", "repo/")];
        assert_eq!(matches, conv_matches(expected_matches));
    }

    #[test]
    fn test_repo_name() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "gcl");
        let expected_matches = vec![
            ("allonsy/gclone/", "allonsy/gclone/"),
            ("aur.archlinux.org:gclone/", "aur.archlinux.org:gclone/"),
        ];
        assert_eq!(matches, conv_matches(expected_matches));
    }

    #[test]
    fn test_user_search() {
        let conf = get_testing_config();
        let matches = get_matches(&conf, "all");
        let expected_matches = vec![
            ("allonsy/", "allonsy/"),
            ("gitlab.com:allonsy/", "gitlab.com:allonsy/"),
        ];
        assert_eq!(matches, conv_matches(expected_matches));
    }
}
