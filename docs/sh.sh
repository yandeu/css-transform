#!/bin/sh

# sh <(curl https://yandeu.github.io/css-transform/sh.sh) index.css bundle.css

current_dir=${PWD}
cd "$current_dir" || exit 1

zip_file="$current_dir/linux.zip"

if ! [ -f "$zip_file" ]; then
  curl -OL https://github.com/yandeu/css-transform/releases/download/v0.0.0-dev.3/linux.zip
  unzip linux.zip
fi

./css-transform "$1" "$2"
