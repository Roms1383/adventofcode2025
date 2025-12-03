test:
    cargo test
lint:
    cargo clippy --fix --allow-dirty
    cargo fix --allow-dirty
    cargo fmt --all

t DAY PART:
    cargo test -p aoc-2025-{{DAY}}-{{PART}}

r DAY PART:
    cargo run -p aoc-2025-{{DAY}}-{{PART}}

ro DAY PART:
    cargo run --release -p aoc-2025-{{DAY}}-{{PART}}
