#!/bin/bash

set -xe
cargo build --release
mv ./target/release/fynder ~/.local/bin/
