SHELL = /bin/bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

build: pkg

pkg: src
	wasm-pack build
	sed -i ".bak" -e 's/"name": "impulse"/"name": "@apolitical\/impulse"/g' pkg/package.json
	rm pkg/package.json.bak

start: build
	(cd www && npm start)
