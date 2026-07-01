build:
    cd ui && pnpm install && pnpm build
    cargo build --release

dev:
    cd frontend && pnpm dev &
    cargo run -p app -- --role all
