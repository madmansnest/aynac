install: release
	cp target/release/aynac ~/bin/

release: target/release/aynac

target/release/aynac:
	cargo build --release

clean:
	rm -rf target/release
