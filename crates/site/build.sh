#!/usr/bin/env sh

set -e

wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
