.PHONY: release clean

release_dir := "./"

release:
	CARGO_TARGET_DIR=${release_dir} cargo build --release

clean:
	cargo clean
	rm -rf release/*
