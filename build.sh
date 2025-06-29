#!/bin/bash
cargo build --release
EXT="so"; [[ $(uname) == "Darwin" ]] && EXT="dylib"
cargo run --bin maestro_client generate --library target/release/libmaestro_client.$EXT --language python --out-dir target/release