# Pretty Make
Make [Make](https://www.gnu.org/software/make/) pretty.

This program intend to extend Make capabilities. Actually it parse a Makefile to produce a pretty help. See example below:

🚧 screenshot

## 🏁 Installation
### Per project by downloading the binary
To use Pretty Make you only have to add the following at the end of your Makefile:

```Makefile
bin/pretty-make:
	@curl -Ls https://raw.githubusercontent.com/awea/pretty-make/master/scripts/install.sh | bash -s

.PHONY: help
## List available commands
help: bin/pretty-make
	@bin/pretty-make pretty-help Makefile
```

### System-wide using cargo
First you have to install Pretty Make using the following command:

```bash
cargo install pretty-make
```

Then you can use Pretty Make in your Makefile like this:

```Makefile
.PHONY: help
## List available commands
help:
  @pretty-make pretty-help Makefile
```

## ⌨️ Usage
```bash
make help
```

## 📝 Makefile help syntax
```Makefile
#@name Project name
#@description Project description. (optional)
#@color-title #70c3cc (optional)
#@color-subtitle #c970cc (optional)
#@color-link #0314fc (optional)

# This is a comment and it won't appear in the `make help`.

## Build site for production use
.PHONY: build
build: deps
  @echo "Building front-end"
  @rm -rf site/*
  @NODE_ENV=production $(WEBPACK) --config webpack/prod.js
  @echo "Front-end built!"

.DEFAULT_GOAL := serve
## Serve:
## - Site at http://localhost:3000 with hot reloading
## - API at http://localhost:3010
## - phpMyAdmin at http://localhost:3011
.PHONY: serve
serve: api deps
  @$(WEBPACK_SERVER) --inline --progress --config webpack/dev.js
```

## ✍️ Authors
- [**@Awea**](https://github.com/Awea) - Idea and initial work
- [**@mmaayylliiss**](https://github.com/mmaayylliiss) - Design and review

## 🤜🤛 Contributing
**Contributions, issues and feature requests are welcome!** See the list of [contributors](../../graphs/contributors) who participated in this project.

## 📄 License
**Pretty Make** is licensed under the [GNU General Public License v3.0](LICENSE).
