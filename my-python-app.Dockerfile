# Build stage
FROM rust:1.86 AS my-app-builder

RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:${PATH}"
RUN env

COPY . /build/

WORKDIR /build/my-python-app
RUN uv sync
RUN uv run maturin develop --uv --manifest-path ../my-rust-crate/Cargo.toml

# Runtime stage
FROM ghcr.io/astral-sh/uv:python3.13-bookworm

WORKDIR /app

COPY my-python-app/ /app
RUN uv sync
COPY --from=my-app-builder /build/my-python-app/.venv/lib/python3.10/site-packages/my_rust_crate /app/.venv/lib/python3.10/site-packages/
COPY --from=my-app-builder /build/my-python-app/.venv/lib/python3.10/site-packages/my_rust_crate-0.1.0.dist-info /app/.venv/lib/python3.10/site-packages/

# Run the application
CMD ["uv", "run", "main.py"] 