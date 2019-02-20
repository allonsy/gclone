mod repo;
mod config;

use std::env;
use std::process::Command;

fn main() {
    let options = parse_args();
    let starting_dir = env::current_dir();
    if starting_dir.is_err() {
        error_out("Unable to retrieve current working directory");
    }

    let repo = repo::RepoUrl::parse(&options.url);
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

fn clone_repo(repo: &repo::RepoUrl) {
    let mut clone_command = Command::new("git");
    clone_command
        .arg("clone")
        .arg(repo.get_clone_url());
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

fn error_out(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

struct Options {
    nocd: bool,
    local: bool,
    url: String
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
                },
                "--nocd" => {
                    options.nocd = true;
                },
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