#!/bin/bash

if [[ $TRAVIS_OS_NAME == "linux" ]]; then
    for COMPLETION_FOLDER in completions/*; do
        if [[ -d $COMPLETION_FOLDER ]]; then
            cp $COMPLETION_FOLDER/* dist/
        fi
    done
fi