
release:
	cargo build --release

install:
	cargo install protobuf && \
	cargo install grpc-compiler