#!/usr/bin/env bash

set -o errexit

cargo build
touch hello-world/src/main.rs
(cd hello-world ; ../target/debug/entr-cargo-bug)
