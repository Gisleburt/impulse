SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

build: pkg

pkg: src
	wasm-pack build
	sed -i ".bak" -e "s/wasm-apolitical-styleguide/@apolitical\/wasm-styleguide/g" pkg/package.json
	rm pkg/package.json.bak

start: build
	(cd www && npm start)
