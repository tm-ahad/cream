#!/bin/bash
cargo build --release
cp ./target/release/nts ./ntc
./ntc build prac