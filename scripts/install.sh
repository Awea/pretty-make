#!/bin/bash

mkdir -p bin

if [ "$(uname)" == "Darwin" ] && [ "$(uname -p)" == "arm" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-macos-arm -o bin/pretty-make
  # See: https://github.com/Awea/pretty-make/issues/17#issuecomment-1252713764
  xattr -d com.apple.quarantine bin/pretty-make
elif [ "$(uname)" == "Darwin" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-macos -o bin/pretty-make
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
  curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-linux -o bin/pretty-make
fi

chmod 755 bin/pretty-make
