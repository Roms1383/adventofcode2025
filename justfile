test:
    cargo test
lint:
    cargo clippy --fix --allow-dirty
    cargo fix --allow-dirty
    cargo fmt --all
run:
    cargo run -p aoc-2025-01-01
    cargo run -p aoc-2025-01-02

t DAY PART:
    cargo test -p aoc-2025-{{DAY}}-{{PART}}

r DAY PART:
    cargo run -p aoc-2025-{{DAY}}-{{PART}}
