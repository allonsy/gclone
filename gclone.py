#!/usr/bin/python3

import sys
import os
from subprocess import run
from urllib.parse import urlparse
from pathlib import Path

GIT_PATH_PREFIX = str(Path.home()) + "/Projects/git"
GIT_DEFAULT_DOMAIN = "github.com"
prefer_https = False

ARGS = sys.argv
NUM_ARGS = len(ARGS)
FLAGS = ARGS[1:-1]
LOCAL_FLAG = False
NOCD_FLAG = False
if "--local" in FLAGS:
    LOCAL_FLAG = True
if "--nocd" in FLAGS:
    NOCD_FLAG = True

def error_out(message):
    print(message, file=sys.stderr)
    print(".") # don't cd anywhere
    sys.exit(1)

def is_https(url):
    if "://" in url:
        return True
    else:
        return False

def is_url(url_str):
    if "://" in url_str or "@" in url_str:
        return True
    else:
        return False
def is_ssh(url):
    return not is_https(url) and is_url(url)

def clone_repo(url):
    cmd = ["git", "clone", url]
    run(cmd)

def get_website_domain(url):
    if is_https(url):
        parsed_url = urlparse(url)
        return parsed_url[1]
    elif is_ssh(url):
        return url.split('@')[1].split(":")[0]
    else:
        return GIT_DEFAULT_DOMAIN


def make_url(domain, fragment):
    if prefer_https:
        return "https://" + domain + "/" + fragment + ".git"
    else:
        return "git@" + domain + ":" + fragment + ".git"

def strip_url_suffix(url):
    if url[-4:] == ".git":
        return url[:-4]
    else:
        return url

def get_path_elements(url):
    new_url = strip_url_suffix(url)
    if is_https(url):
        parsed_url = urlparse(new_url)
        path = parsed_url[2].split('/')
        return [item for item in path if item != ""]
    elif is_ssh(url):
        parsed_url = new_url.split(":")[1]
        return parsed_url.split("/")
    else:
        return url.split("/")

def list_subdirs(path_str):
    subfiles = os.listdir(path_str)
    dirs = []
    for fname in subfiles:
        if os.path.isdir(os.path.join(path_str, fname)):
            dirs.append(fname)
    
    return dirs

def find_ignore_case(ls, to_find):
    for elem in ls:
        if elem.lower() == to_find.lower():
            return elem
    return None

def chdir_to_path(path):
    starting_path = GIT_PATH_PREFIX
    for path_elem in path:
        found_path = find_ignore_case(list_subdirs(starting_path), path_elem)
        new_path = os.path.join(starting_path, path_elem)
        if found_path == None:
            os.mkdir(new_path)
        os.chdir(new_path)
        starting_path = new_path

def chdir_to_input(input):
    path_elems = [get_website_domain(input)]
    path_elems += get_path_elements(input)

    chdir_to_path(path_elems[:-1])

def clone_repo_from_input(input, local=False):
    if not local:
        chdir_to_input(input)
    
    parsed_input = get_path_elements(input)
    expected_repo_name = parsed_input[-1]

    actual_repo_name = find_ignore_case(list_subdirs("."), expected_repo_name)
    if actual_repo_name != None:
        if not NOCD_FLAG:
            print(os.path.join(os.getcwd(), actual_repo_name))
            sys.exit(0)
        else:
            error_out("Directory already exists, will not clone")
    
    url = ""
    if is_url(input):
        url = input
    else:
        url = make_url(GIT_DEFAULT_DOMAIN, input)
    clone_repo(url)
    actual_repo_name = find_ignore_case(list_subdirs("."), expected_repo_name)
    return os.path.join(os.getcwd(), actual_repo_name)


# main


if NUM_ARGS <= 1:
    error_out("Please provide a url or repo to clone!")
elif NUM_ARGS == 2:
    print(clone_repo_from_input(ARGS[1]))
    sys.exit(0)
else:
    url = ARGS[-1]
    cd_dir = clone_repo_from_input(url, LOCAL_FLAG)
    if NOCD_FLAG:
        print(".")
    else:
        print(cd_dir)