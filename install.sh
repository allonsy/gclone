#!/bin/bash

echo "Symlinking binaries"
sudo ln -svf `pwd`/gclone.py /usr/local/bin/gclone.py
sudo ln -svf `pwd`/gclone /usr/local/bin/gclone

echo "Symlinks made. Please add the following alias to your shell rc file:"
echo "alias gclone='source gclone'"