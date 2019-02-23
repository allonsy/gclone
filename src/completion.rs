use super::config;
use std::fs;
use glob::glob;

pub fn get_matches(input: &str) -> Vec<String> {

}

fn get_all_tlds(conf: &config::Config) -> Vec<String> {
    let base_path = conf.get_base_path();
    Vec<String> tlds = Vec::new();

    let cur_dir_res = fs::read_dir(base_path);
    if cur_dir_res.is_err() {
        return tlds;
    }
    let base_dir = cur_dir_res.unwrap();
    for dir in base_dir {
        if !dir.is_err() {
            let dir_path = dir.unwrap();
            if dir_path.path().is_dir() {
                tlds.push(dir_path.to_str().unwrap().to_string());
            }
        }
    }

    return tlds;
}

fn scan_single_tld(conf: &config::Config, tld: &str, path: &str) -> Vec<String> {
    let base_path = conf.get_base_path();
    let path_elems: Vec<&str> = path.split("/");

    let full_path = 
        format!("{}/{}/{}*", base_path.to_str().unwrap(), tld, path);
    

}
