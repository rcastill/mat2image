RS_FILES=$(shell find . -name '*.rs' -type f)

save_as: $(RS_FILES)
	cargo build --release --example save_as
	[ ! -f save_as ] && ln -s target/release/examples/save_as save_as

clean:
	rm -f save_as