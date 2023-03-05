#!/bin/zsh

# This script creates req'd functions in HOME directory
# and adds them to .zshrc
#
# This is intended to use on OSX only.
#
# Author: Bartosz Lukasik, 2023

os_name=$(uname)

if [ "$os_name" != "Darwin" ]; then
    echo "ERR: Detected system is $os_name - Cannot run the script"
    exit 0
fi

dir="$HOME/vault"

echo "INFO: Installation started in HOME directory ($HOME) !!!"

if [ ! -d "$dir" ]; then
    mkdir "$dir"
    echo "INFO: Successfully created directory: $dir"
    else
      echo "ERR: Directory $dir already exists"
      exit 0
fi

echo "INFO: Building application artifacts..."
cargo build --release
echo "INFO: Application artifacts built successfully!"

echo "INFO: Copying application artifacts..."
cp ./../target/release/vault "$dir/vault"
echo "INFO: Application artifacts copied successfully!"

echo "INFO: Copying ./run.sh to $dir/run.sh"
cp ./run.sh "$dir/run.sh"
echo "INFO: ./run.sh copied to $dir/run.sh"

echo "INFO: Appending source to $HOME/.zshrc"
cat ./zshrc_append.sh >> "$HOME/.zshrc"
echo "INFO: Source appended to $HOME/.zshrc successfully"

echo "INFO: Applying .zshrc changes"
source "$HOME/.zshrc"

echo "INSTALLATION COMPLETE (you may need to restart your shell session)"