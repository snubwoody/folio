set windows-shell := ["powershell"]

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

lint:
    cargo clippy --all-targets --all-features --fix
    cargo fmt
    pnpm lint:fix

lint-web:
    pnpm eslint . --max-warnings 0
#    pnpm stylelint **/*.{css,svelte} --max-warnings 0
