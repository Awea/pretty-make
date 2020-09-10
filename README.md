# Pretty Make
An attempt to make [Make](https://www.gnu.org/software/make/) pretty.

🚧 For now it's only a replacement for a buggy `awk` command that I copy/paste in every project to have a pretty `make help`.

## 🏁 Installation
To use Pretty Make you only have to add the following at the end of your Makefile:

```Makefile
bin/pretty-make:
	bash <(curl -Ls https://raw.githubusercontent.com/awea/pretty-make/master/scripts/install.sh)

.PHONY: help
## List available commands
help: bin/pretty-make
	@bin/pretty-make Makefile
```

## ⌨️ Usage
```bash
make help
```

## 📝 Makefile help syntax
```Makefile
#@name Name of your project
#@description A short description of your project (optional)

## A description of my awesome-command
command:
	...

# A comment (which won't appear in the help)

## A description of my other command
## on multiple lines!
another-command:
	...
```

## ✍️ Authors
- [**@Awea**](https://github.com/Awea) - Idea and initial work

## 🤜🤛 Contributing
**Contributions, issues and feature requests are welcome!** See the list of [contributors](../../graphs/contributors) who participated in this project.

## 📄 License
**Pretty Make** is licensed under the [GNU General Public License v3.0](LICENSE).
