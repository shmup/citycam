release:
  cargo build --release
  upx target/release/citycam
  ls -lah target/release/citycam
