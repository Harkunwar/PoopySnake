#!/bin/bash

curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
PATH=\"$HOME/.cargo/bin:$PATH\" curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
yarn build