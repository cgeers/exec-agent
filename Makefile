PROJECT := exec-agent

release: build-ui
	cargo build --release

build: build-ui
	cargo build

build-ui:
	$(MAKE) -C ui build