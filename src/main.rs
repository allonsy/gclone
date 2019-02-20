mod config;
mod repo;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

static mut GLOBAL_ROLLBACK: Option<Vec<PathBuf>> = None;
static mut STARTING_DIR: Option<PathBuf> = None;

fn main() {
    let options = parse_args();
    let starting_dir = env::current_dir();
    if starting_dir.is_err() {
        error_out("Unable to retrieve current working directory");
    }
    unsafe {
        STARTING_DIR = Some(starting_dir.as_ref().unwrap().clone());
    }

    let repo = repo::Repo::parse(&options.url);
    let full_path = repo.get_fs_path();

    if !options.nocd {
        if full_path.exists() {
            println!("{}", full_path.to_str().unwrap());
            std::process::exit(0);
        }
    }

    if !options.local {
        let target_dir = full_path.parent();
        if target_dir.is_none() {
            error_out("root repo cannot be used");
        }
        mkdir(target_dir.as_ref().unwrap());
        let cwd_result = env::set_current_dir(target_dir.unwrap());
        if cwd_result.is_err() {
            error_out("Unable to cd to clone directory");
        }
    }
    clone_repo(&repo);

    if options.nocd {
        println!("{}", starting_dir.unwrap().to_str().unwrap());
    } else {
        println!("{}", full_path.to_str().unwrap());
    }
}

fn clone_repo(repo: &repo::Repo) {
    let mut clone_command = Command::new("git");
    clone_command.arg("clone").arg(repo.get_clone_url());
    let status = clone_command.spawn();
    if status.is_err() {
        error_out("Failed to spawn git clone process");
    }
    let status_code = status.unwrap().wait();
    if status_code.is_err() {
        error_out("Unable to find child process");
    }
    if !status_code.unwrap().success() {
        error_out("Git clone process errored out!");
    }
}

fn mkdir(path: &Path) {
    let mut cur_path = path;
    while !cur_path.exists() {
        unsafe {
            if GLOBAL_ROLLBACK.is_none() {
                GLOBAL_ROLLBACK = Some(vec![PathBuf::from(cur_path)]);
            } else {
                GLOBAL_ROLLBACK
                    .as_mut()
                    .unwrap()
                    .push(PathBuf::from(cur_path));
            }
        }
        cur_path = cur_path.parent().unwrap();
    }
    let res = std::fs::create_dir_all(path);
    if res.is_err() {
        error_out("Unable to create directory structure");
    }
}

fn error_out(msg: &str) -> ! {
    unsafe {
        if GLOBAL_ROLLBACK.is_some() {
            rollback();
        }
    }
    eprintln!("{}", msg);
    unsafe {
        if STARTING_DIR.is_some() {
            println!("{}", STARTING_DIR.as_ref().unwrap().display());
        }
    }
    std::process::exit(1);
}

unsafe fn rollback() {
    for path in GLOBAL_ROLLBACK.as_ref().unwrap() {
        let _ = std::fs::remove_dir_all(path);
    }
}

struct Options {
    nocd: bool,
    local: bool,
    url: String,
}

fn parse_args() -> Options {
    let args: Vec<String> = env::args().collect();
    let mut options = Options {
        nocd: false,
        local: false,
        url: String::new(),
    };

    for arg in &args[1..] {
        if &arg[0..2] == "--" {
            match arg.as_ref() {
                "--local" => {
                    options.local = true;
                }
                "--nocd" => {
                    options.nocd = true;
                }
                _ => {
                    error_out(&format!("Unknown arg: {}", arg));
                }
            }
        } else {
            options.url = arg.clone();
        }
    }

    if options.url.is_empty() {
        error_out("Please provide a URL!");
    }
    options
}
