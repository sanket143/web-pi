run:
	cargo run

pi:
	PKG_CONFIG_PATH=/usr/aarch64-linux-gnu/lib/ PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/ cargo build --target aarch64-unknown-linux-gnu --verbose --release
