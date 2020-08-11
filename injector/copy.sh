#!/usr/bin/env sh

set -e

cp -v pkg/bundle.js ../server/static/pkg/injector-bundle.js
cp -v pkg/reform_injector_bg.wasm ../server/static/pkg/reform_injector.wasm
