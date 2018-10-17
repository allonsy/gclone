# gclone
A simple git clone command line helper

# Installation
run the install script: `./install.sh`. It will require sudo privledges

You will need to add the following alias to your shell rc file:

`alias gclone='source gclone'`

# Usage
To use `gclone` run gclone with a repo name. Repo names take the following forms:

* `allonsy/gclone` (assumes github.com and ssh protocol for cloning)
* `github.com/allonsy/gclone` (assumes ssh protocol)
* `git@github.com:allonsy/gclone.git` (assumes nothing)
* `https://github.com/allonsy/gclone.git` (assumes nothing)

Therefore, with these repo names in mind, the gclone command is simply:

`gclone [FLAGS] [REPO_NAME]` (replacing `[REPO_NAME]` with your desired repo name). FLAGS are described below

`gclone` will checkout the repo to your tree location specified in the `gclone.py` file. 

For example, my tree is located at `~/Projects/git`. Therefore, this repo would be clone to: `~/Projects/git/github.com/allonsy/gclone`

Changing the `GIT_PATH_PREFIX` variable in `gclone.py` will change the location accordingly to your desired structure

`gclone` will autocd into the newly cloned repo. To turn this off, see the flags section

If the repo is already cloned in the target location, `gclone` won't reclone, but it will autocd into the target location (so `gclone` can then be used as a navigator of your local file tree)

## Flags

* `--nocd` : `gclone` will automatically cd into the cloned directory, to disable this, pass the flag `--nocd` BEFORE THE REPO NAME. E.g `gclone --nocd allonsy/gclone`
* `--local` : Tells gclone to not clone in the standard tree location but rather in the current working directory. `gclone` will still auto cd into the new directory after cloning. Compose this flag with `--nocd` to also not cd into the new directory. Just like `nocd` this flag must be placed before the repo name (although flag ordering doesn't matter)

# Customizations
* `gclone` defaults to using ssh protocol by default (unless specified by the url). To Change this to https, change the `prefer_https` variable in `gclone.py`
* `gclone` defaults to `github.com` when the repo name doesn't specify, to use another server (like gitlab), change the `GIT_DEFAULT_DOMAIN` flag in `gclone.py`

# Contribution
* All Contributions, Bugs, and suggestions are welcome, just fill out an issue or PR
