# pyo3 demo

To run the code in this demo:

```bash
cd my-python-app
uv run maturin develop --uv --manifest-path ../my-rust-crate/Cargo.toml && uv run main.py
```

To run it with hot reload:

```bash
cd my-rust-crate
cargo watch --watch .. --workdir ../my-python-app -- bash -c 'uv run maturin develop --uv --manifest-path ../my-rust-crate/Cargo.toml && uv run main.py'
```