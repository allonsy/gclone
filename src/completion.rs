use super::config;
use glob::glob;
use std::fs;
use std::path::PathBuf;

pub fn get_matches(input: &str) -> Vec<(String, String)> {
    let conf = config::get_config();

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

    return search_for_component(&conf, input);
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

    return new_tlds;
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
        if !dir.is_err() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                tlds.push(format!(
                    "{}",
                    dir_path.path().file_name().unwrap().to_str().unwrap()
                ));
            }
        }
    }

    return tlds;
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
                    format!("{}:{}", tld, rel_path.display())
                } else {
                    format!("{}", rel_path.display())
                };
                let short_name = if add_short_desc {
                    format!("{}", glob_path.file_name().unwrap().to_str().unwrap())
                } else {
                    input_name.clone()
                };
                completions.push((input_name, short_name));
            }
        }
    }
    return completions;
}

fn get_top_level_hints(conf: &config::Config) -> Vec<(String, String)> {
    let mut hints = Vec::new();

    let non_default_tlds = get_all_non_default_tlds(conf);
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
        if !dir.is_err() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                let file_name = format!("{}", dir_path.file_name().to_str().unwrap());
                hints.push((file_name.clone(), file_name));
            }
        }
    }

    return hints;
}

fn search_for_component(conf: &config::Config, input: &str) -> Vec<(String, String)> {
    let mut hints = Vec::new();
    let default_domain = conf.get_domain();
    let max_depth = conf.get_default_search_depth();

    let base_path = conf.get_base_path();
    let default_path = format!("{}", base_path.join(default_domain).display());
    let default_hints = list_components(&default_path, "", input, 0, max_depth);

    hints.push((
        format!("{}:", default_domain),
        format!("{}:", default_domain),
    ));

    for (hint, short) in default_hints {
        hints.push((hint.clone(), short.clone()));
        hints.push((
            format!("{}:{}", default_domain, hint),
            format!("{}:{}", default_domain, short),
        ));
    }

    for tld in get_all_non_default_tlds(conf) {
        let search_path = format!("{}/{}", base_path.display(), tld);
        for hint in list_components(&search_path, &tld, input, 0, max_depth) {
            hints.push(hint);
        }
    }

    return hints;
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
        if !dir.is_err() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                let file_name = format!("{}", dir_path.file_name().to_str().unwrap());
                if file_name.starts_with(input) {
                    let hint_name = format!("{}{}", prefix, file_name);
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
    return hints;
}
