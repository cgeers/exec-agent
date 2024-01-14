PROJECT := exec-agent

build: build-ui
	cargo build

release: build-ui
	cargo build --release

build-ui:
	$(MAKE) -C ui build

run: build-ui
	cargo run