# gclone [![Build Status](https://travis-ci.org/allonsy/gclone.svg?branch=master)](https://travis-ci.org/allonsy/gclone)
A simple git clone command line helper

# Installation
run the install script: `./install.sh`. It will require sudo privledges

You will need to add the following alias to your shell rc file:

`alias gclone='source gclone'`

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
* Any of these options can be omitted and can be written in any order
* the config is a TOML file and must adhere to the toml spec

# Contribution
* All Contributions, Bugs, and suggestions are welcome, just fill out an issue or PR
