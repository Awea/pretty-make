#!/bin/bash

mkdir -p bin

if [ "$(uname)" == "Darwin" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-macos -o bin/pretty-make
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-linux -o bin/pretty-make
fi

chmod 755 bin/pretty-make
