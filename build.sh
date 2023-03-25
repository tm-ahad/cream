RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build --release
cp ./target/release/nts ./ntc
./ntc build prac