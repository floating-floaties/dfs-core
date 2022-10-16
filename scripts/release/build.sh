
set -e

#cargo update -p string-utility
#cargo update -p eval-utility
#cargo update -p inflection-rs
#cargo update -p resolver

#rustup target add armv7-unknown-linux-gnueabihf
#rustup target add x86_64-unknown-linux-gnu
#rustup target add x86_64-pc-windows-gnu
#rustup target add aarch64-unknown-linux-gnu

for target in "x86_64-unknown-linux-gnu"; do
  cargo test --target "$target" --release
  cargo build --target "$target" --release
  mkdir -p "./bin/$target"
  cp "./target/$target/release/dfs" "./bin/$target/dfs"
done
