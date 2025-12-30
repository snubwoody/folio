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

# Run the desktop app in dev mode
dev-desktop:
    cargo tauri dev

# Run the website in dev mode
dev-web:
    pnpm --filter web dev