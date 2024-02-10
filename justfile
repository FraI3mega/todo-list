build-all-release:
	cargo nextest run
	cargo zigbuild --target x86_64-pc-windows-gnu --release
	cargo build --release
