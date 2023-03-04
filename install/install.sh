#!/bin/zsh

# This script creates req'd functions in HOME directory
# and adds them to .zshrc
#
# This is intended to use on OSX only.
#
# Author: Bartosz Lukasik, 2023

dir="$HOME/vault"

echo "INFO: Installation started in HOME directory ($HOME) !!!"

if [ ! -d "$dir" ]; then
    mkdir "$dir"
    echo "INFO: Successfully created directory: $dir"
    else
      echo "ERR: Directory $dir already exists"
      exit 0
fi

# TODO #1: Copy run.sh to dir
# TODO #2: Update .zshrc to have copied file as source