test:
    cargo test
lint:
    cargo clippy --fix --allow-dirty
    cargo fix --allow-dirty
    cargo fmt --all
run:
    cargo run -p aoc-2025-01-01
    cargo run -p aoc-2025-01-02
