set shell := ["powershell"]


# Create a new sqlite database
db-create:
    cargo sqlx database create

# Run migrations on the sqlite database
db-migrate:
    cargo sqlx migrate run --source crates/folio-desktop/migrations

db-setup:
    cargo sqlx database drop -y
    just db-create
    just db-migrate