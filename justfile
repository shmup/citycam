release:
  cargo build --release
  upx target/release/citycam
  ls -lah target/release/citycam

docker-build:
    docker compose build
    docker create --name temp-citycam-container citycam-citycam
    mkdir -p ./target/release
    docker cp temp-citycam-container:/app/target/release/citycam ./target/release/
    docker rm temp-citycam-container
    chmod +x ./target/release/citycam
    ls -lah target/release/citycam

release-docker: docker-build
    upx target/release/citycam
    ls -lah target/release/citycam

