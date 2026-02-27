set windows-shell := ["nu","-c"]

[working-directory: "crates/folio-desktop"]
add-migration name:
    cargo sqlx migrate add {{name}}

# Run migrations on the sqlite database
migrate:
    cargo sqlx migrate run --source crates/folio-desktop/migrations

# Create a new sqlite database
db-create:
    cargo sqlx database create

# Run migrations on the sqlite database
db-migrate:
    cargo sqlx migrate run --source crates/folio-desktop/migrations

# Setup a local sqlite database
db-setup:
    cargo sqlx database drop -y
    just db-create
    just db-migrate

# Run the app in dev mode
dev:
    cargo tauri dev

# Test all the crates (rust code)
test-backend:
    cargo nextest run --no-fail-fast --all-targets --all-features

# Test the frontend code (headless)
test-frontend:
    pnpm -F @folio/desktop test:headless

# Test the frontend in browser mode
test-browser:
    pnpm -F @folio/desktop test:browser

# Format and lint all the code
lint:
    cargo clippy --all-targets --all-features --allow-dirty --fix
    cargo fmt
    pnpm lint:fix
