#!/bin/zsh

# This script should be copied to vault app directory.
# .zshrc file should have this as source to allow running
# from anywhere within the system.
#
# This is intended to use on OSX only.
#
# Author: Bartosz Lukasik, 2023

dir="$HOME/vault"

function vault() {
  if [ "$2" ]; then
      "$dir/vault" "$1" "$2"
      else
        "$dir/vault" "$1"
  fi
}
