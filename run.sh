#!/bin/bash

# This passes all arguments given to the script (like --release) to cargo run
RUSTFLAGS="-Awarnings" cargo run "$@"
