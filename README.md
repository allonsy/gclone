# gclone [![Build Status](https://travis-ci.org/allonsy/gclone.svg?branch=master)](https://travis-ci.org/allonsy/gclone)
A simple git clone command line helper

# Installation

## Manual
* First, build the project by running `cargo build --release`
* Then, copy or symlink the binary somewhere onto your path (like `/usr/local/bin`). The binary is located under: `target/release/gclone-bin`
* You could also run `cargo install` if `~/.cargo/bin` is on your path

### Shell installation
* After installing, you will need to add the following shell function to your shellrc file (or you may add it to any file that is sourced when your shell is run)

For `bash` or `zsh`:

```
function gclone {
    cd `gclone-bin $@`
}
```

For `fish`:

```
function gclone
    cd (gclone-bin $argv)
end
```

### Shell Autocompletions
To install autocompletions, please run the following commands:

* zsh: `cp completions/zsh/_gclone ~/.config/zsh/completions`
* fish: `cp completions/fish/gclone.fish ~/.config/fish/completions`

for `zsh` you will also need to add the following line to your `~/.zshrc` file before the `compinit` line:
* `fpath=(~/.config/zsh/completions $fpath)` to add the `~/.config/zsh/completions` directory to your completions search path

For `fish`, you will need to make sure that `~/.config/fish/completions` is on your completion path as well. As long as `XDG_CONFIG_HOME`, the `~/.config/fish/completions` directory should suffice. However, if completions aren't working, you might want to try dumping `gclone.fish` into `/usr/share/fish/completions` or `/usr/share/fish/completions/vendor_completions.d`. 

## OS-Level Installation

### AUR Package
* Aur package name: `gclone`
* It is available via your favorite AUR installation method.
* Package page: [gclone aur page](https://aur.archlinux.org/packages/gclone/)
* You will still need to install the gclone function to your shell rc file (although shell completions are auto-installed)

### Homebrew Package
Coming soon!

# Usage
To use `gclone` run gclone with a repo name. Repo names take the following forms:

* `allonsy/gclone` (assumes github.com and ssh protocol for cloning)
* `github.com:allonsy/gclone` (assumes ssh protocol)
* `git@github.com:allonsy/gclone.git` (assumes nothing)
* `https://github.com/allonsy/gclone.git` (assumes nothing)

Therefore, with these repo names in mind, the gclone command is simply:

`gclone [FLAGS] [REPO_NAME]` (replacing `[REPO_NAME]` with your desired repo name). FLAGS are described below

`gclone` will checkout the repo to your tree location specified in the `gclone.py` file. 

For example, my tree is located at `~/Projects/git`. Therefore, this repo would be clone to: `~/Projects/git/github.com/allonsy/gclone`

Changing the `basePath` variable in `gclone`'s config will change the location accordingly to your desired structure

`gclone` will autocd into the newly cloned repo. To turn this off, see the flags section

If the repo is already cloned in the target location, `gclone` won't reclone, but it will autocd into the target location (so `gclone` can then be used as a navigator of your local file tree)

## Flags

* `--nocd` : `gclone` will automatically cd into the cloned directory, to disable this, pass the flag `--nocd`. E.g `gclone --nocd allonsy/gclone`
* `--local` : Tells gclone to not clone in the standard tree location but rather in the current working directory. `gclone` will still auto cd into the new directory after cloning. Compose this flag with `--nocd` to also not cd into the new directory.

### Helper flags
These flags are helper flags used in other programs (like shell completion scripts). They can be used to write meta-scripts over `gclone`.
* `--get-base-dir`: prints the base directory
* `--get-base-domain`: prints the default domain
* `--match-prefix [ARG]`: prints the shell completions for `$ARG`

# Customizations
You can override some of the basic values via a config file called one of the following:
* `~/.config/gclone/gclone.toml` (Linux only)
* `XDG_CONFIG_HOME/gclone/gclone.toml` (Linux only)
* `$HOME/Library/Preferences/gclone/gclone.toml` (MacOS only)
* `{FOLDERID_RoamingAppData}\gclone\gclone.toml` (windows only)
* `$GCLONE_CONF_FILE` (cross platform)

The toml file should like the following:
```toml
basePath = "/home/user/fooo/bar"
defaultDomain = "gitlab.com"
defaultHttps = true
```

* `basePath` is the default location where cloning occurs. It defaults to `$HOME/Projects/git`. The path provided must be absolute and doesn't support shortcuts like `~`
* defaultDomain is the default domain for fetching repos. The default is `github.com`
* `defaultHttps` is a boolean value which tells `gclone` to use https when the protocol cannot be infered. It defaults to false (defaults to using ssh)
* `defaultDepth` is an integer value which tells `gclone` how far down to search for repos when shell autocompleting. The default is `2` which matches all domains that use a `user/repo` convention (like github and gitlab). Unless you are using some self hosted domain with top level repos, this value shouldn't be changed.
* Any of these options can be omitted and can be written in any order
* the config is a TOML file and must adhere to the toml spec

# Contribution
* All Contributions, Bugs, and suggestions are welcome, just fill out an issue or PR
* Please ensure that all submitted code passes linting via `cargo clippy` and is run through `rustfmt` via `cargo fmt`.
* All code changes should pass the existing tests in addition to any new tests added. Tests are checked via `cargo test`
