#!/bin/bash

CURRENT_VER=$(head Cargo.toml | grep version | cut -f2 -d'=' | cut -f2 -d\")

# rest of the platforms binary
cargo b --release --example records-gen --target aarch64-apple-darwin
cargo b --release --example records-gen --target x86_64-apple-darwin
cargo b --release --example records-gen --target x86_64-pc-windows-gnu
cargo b --release --example records-gen --target aarch64-unknown-linux-gnu
cargo b --release --example records-gen --target x86_64-unknown-linux-gnu

# remove existing files
rm -rf tmp
# make the folder again
mkdir -p tmp

# copy files to the tmp folder
# win
cp target/x86_64-pc-windows-gnu/release/examples/records-gen.exe tmp/records-gen_x86-64.exe
# macos
cp target/aarch64-apple-darwin/release/examples/records-gen tmp/records-gen_macos_aarch64
cp target/x86_64-apple-darwin/release/examples/records-gen tmp/records-gen_macos_x86-64
# linux
cp target/aarch64-unknown-linux-gnu/release/examples/records-gen tmp/records-gen_linux_aarch64
cp target/x86_64-unknown-linux-gnu/release/examples/records-gen tmp/records-gen_linux_x86-64

# create the new zip files
cd tmp
zip -9r records-gen"$CURRENT_VER"-windows.zip records-gen_x86-64.exe
zip -9r records-gen"$CURRENT_VER"-macos.zip records-gen_macos_aarch64 records-gen_macos_x86-64
zip -9r records-gen"$CURRENT_VER"-linux.zip records-gen_linux_aarch64 records-gen_linux_x86-64
cd ..

# delete the tmp files
rm -f tmp/records-gen_x86-64.exe tmp/records-gen_macos_aarch64 tmp/records-gen_macos_x86-64 tmp/records-gen_linux_aarch64 tmp/records-gen_linux_x86-64