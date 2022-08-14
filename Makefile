RS_FILES=$(shell find . -name '*.rs' -type f)

save_as: Cargo.toml $(RS_FILES)
	cargo build --release --example save_as
	ln -fs target/release/examples/save_as save_as

clean:
	rm -f save_as