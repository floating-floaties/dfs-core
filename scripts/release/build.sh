
for target in "x86_64-unknown-linux-gnu"; do
  cargo test --target "$target" --release
  cargo build --target "$target" --release
  mkdir -p "./bin/$target"
  cp "./target/$target/release/dfs" "./bin/$target/dfs"
done
