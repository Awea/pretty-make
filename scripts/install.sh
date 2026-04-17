#!/bin/bash

install_path="${INSTALL_PATH:-bin}"

mkdir -p $install_path

if [ "$(uname)" == "Darwin" ] && [ "$(uname -p)" == "arm" ]; then
	curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-macos-arm -o "${install_path}/pretty-make"
	# See: https://github.com/Awea/pretty-make/issues/17#issuecomment-1252713764
	xattr -d com.apple.quarantine "${install_path}/pretty-make"
elif [ "$(uname)" == "Darwin" ]; then
	curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-macos -o "${install_path}/pretty-make"
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
	curl -L https://github.com/Awea/pretty-make/releases/latest/download/pretty-make-linux -o "${install_path}/pretty-make"
fi

chmod 755 "${install_path}/pretty-make"
