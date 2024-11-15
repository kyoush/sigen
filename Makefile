.PHONY: release clean

release_dir := "./"

release:
	cargo build --release
	cargo build --target x86_64-pc-windows-gnu --release
	cp target/release/sigen release/
	cp target/x86_64-pc-windows-gnu/release/sigen.exe release/

clean:
	cargo clean
	rm -rf release/*
