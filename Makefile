sources = python src

.PHONY: .uv  # Check that uv is installed
.uv:
	@uv -V || echo 'Please install uv: https://docs.astral.sh/uv/getting-started/installation/'

.PHONY: install  ## Install the package, dependencies
install: .uv
	uv sync --frozen --all-groups --all-packages --all-extras

.PHONY: .maturin  # Check that maturin is installed
.maturin:
	@command -v maturin > /dev/null || echo 'Please install maturin: https://www.maturin.rs/installation.html'

.PHONY: .rust  # Check that Rust is installed
.rust:
	@rustup --version || echo 'Please install Rust: https://rust-lang.org/tools/install/'

.PHONY: format  ## Auto-format source files
format: .uv .rust
	uv run ruff check --fix $(sources)
	uv run ruff format $(sources)
	cargo fmt

.PHONY: lint-python  ## Lint python source files
lint-python: .uv
	uv run ruff check $(sources)
	uv run ruff format --check $(sources)

.PHONY: generate-stubs  ## Generate type stubs for the package
generate-stubs: .rust
	# cargo run --bin stub_gen
	maturin generate-stubs --out python/app -m crates/python-bindings/Cargo.toml
	# uv run python inject_docstring.py
	find python/app -type f -name "*.pyi" -exec uv run ruff format {} +

.PHONY: help  ## Display this message
help:
	@grep -E \
		'^.PHONY: .*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ".PHONY: |## "}; {printf "\033[36m%-19s\033[0m %s\n", $$2, $$3}'

.PHONY: develop ## Update the dynamic library and stubs in the python package
develop: .maturin
	maturin develop -m crates/python-bindings/Cargo.toml
	$(MAKE) generate-stubs

