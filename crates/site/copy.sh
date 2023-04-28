#!/usr/bin/env sh

set -e

cp -v pkg/bundle.js ../server/static/pkg
cp -v pkg/reform_site_bg.wasm ../server/static/pkg/reform_site.wasm
