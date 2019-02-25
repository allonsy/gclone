use dirs;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use toml::Value;

static GCLONE_CONF_FILE_VAR_NAME: &'static str = "GCLONE_CONF_FILE";

pub struct Config {
    base_path: PathBuf,
    default_https: bool,
    default_domain: String,
    default_search_depth: usize,
}

impl Config {
    fn new() -> Config {
        Config {
            base_path: get_default_base_path(),
            default_domain: get_default_domain().to_string(),
            default_https: get_default_is_https(),
            default_search_depth: get_default_search_depth(),
        }
    }

    pub fn get_base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn get_domain(&self) -> &String {
        &self.default_domain
    }

    pub fn get_is_https(&self) -> bool {
        self.default_https
    }

    pub fn get_default_search_depth(&self) -> usize {
        self.default_search_depth
    }
}

pub fn get_config() -> Config {
    let mut conf = Config::new();
    let config_file_env_var = env::var(GCLONE_CONF_FILE_VAR_NAME);

    let config_file_path =
        if config_file_env_var.is_ok() && config_file_env_var.as_ref().unwrap() != "" {
            let fpath_str = config_file_env_var.unwrap();
            PathBuf::from(fpath_str)
        } else {
            let conf_dir = dirs::config_dir();
            if conf_dir.is_none() {
                return conf;
            }

            let mut config_file_path = conf_dir.unwrap();
            config_file_path.push("gclone");
            config_file_path.push("gclone.toml");
            config_file_path
        };
    if !config_file_path.exists() {
        return conf;
    }

    let config_file_contents = std::fs::read_to_string(config_file_path);
    if config_file_contents.is_err() {
        eprintln!("gclone config error: Unable to read config file");
        return conf;
    }

    let parsed_val = config_file_contents.unwrap().parse::<Value>();
    if parsed_val.is_err() {
        eprintln!("gclone config error: Unable to parse config file");
        return conf;
    }

    let parsed_toml = parsed_val.unwrap();
    if parsed_toml.is_table() {
        for (key, val) in parsed_toml.as_table().unwrap() {
            parse_value(&mut conf, key, val);
        }
    }

    conf
}

fn parse_value(conf: &mut Config, key_name: &str, val: &Value) {
    if val.is_str() {
        let val_str = val.as_str().unwrap();
        match key_name {
            "basePath" => {
                let path_parse = PathBuf::from_str(val_str);
                if path_parse.is_err() {
                    eprintln!("gclone config error: Unrecognizable path: {}", val_str);
                } else {
                    conf.base_path = path_parse.unwrap();
                }
            }
            "defaultDomain" => {
                conf.default_domain = val_str.to_string();
            }
            _ => {}
        }
    } else if val.is_bool() && key_name == "defaultHttps" {
        conf.default_https = val.as_bool().unwrap();
    } else if val.is_integer() && key_name == "defaultDepth" {
        conf.default_search_depth = val.as_integer().unwrap() as usize;
    }
}

fn get_default_base_path() -> PathBuf {
    let mut base_path = PathBuf::from("~");
    let home_opt = dirs::home_dir();
    if home_opt.is_some() {
        base_path = home_opt.unwrap();
    }
    base_path.push("Projects");
    base_path.push("git");
    base_path
}

fn get_default_domain() -> &'static str {
    "github.com"
}

fn get_default_is_https() -> bool {
    false
}

fn get_default_search_depth() -> usize {
    2
}
