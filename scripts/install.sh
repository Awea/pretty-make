#!/bin/bash

if [ ! -d "$PWD/bin" ]; then
  mkdir bin
fi

if [ "$(uname)" == "Darwin" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-macos -o pretty-make
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-linux -o pretty-make
fi

chmod 755 pretty-make
mv pretty-make bin/pretty-make
