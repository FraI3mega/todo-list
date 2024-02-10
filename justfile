build-all-release:
	cargo nextest run
	cargo zigbuild --target x86_84-pc-windows-gnu --release
	cargo build --release
