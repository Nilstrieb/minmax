#!/usr/bin/env bash

if ! which rustc > /dev/null;
then
    echo "ERROR: Rust must be installed: https://www.rust-lang.org/tools/install"
    exit 1
fi

echo "${BASH_SOURCE[0]}"

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

echo "Building Rust library..."
cargo build --release "--manifest-path=$SCRIPT_DIR/Cargo.toml"

export LD_LIBRARY_PATH="$SCRIPT_DIR/target/release"
echo "Setting LD_LIBRARY_PATH to $LD_LIBRARY_PATH"

echo "Running Java tests. If this fails, there's something wrong :/"
cd "$SCRIPT_DIR/minmax-java"
./gradlew build

echo
echo "To use this player in another project, set the following environment variable:"
echo
echo "export LD_LIBRARY_PATH=\"$SCRIPT_DIR/target/release\""
