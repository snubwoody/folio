set windows-shell := ["nu","-c"]

# Create a new sql migration
[working-directory: "crates/folio-desktop"]
add-migration name:
    sqlx migrate add {{name}}

# Run migrations on the sqlite database
migrate:
    sqlx migrate run --source crates/folio-desktop/migrations

# Create a new sqlite database
db-create:
    sqlx database create

# Run migrations on the sqlite database
db-migrate:
    sqlx migrate run --source crates/folio-desktop/migrations

# Setup a local sqlite database
db-setup:
    sqlx database drop -y
    just db-create
    just db-migrate

# Generate a CMakeLists.txt file for IDE support
gen-cmake:
    xmake project -k cmakelists

# Generate a compile_commands.json file for IDE support
gen-compile-commands:
    xmake project -k compile_commands
# Run the app in dev mode
dev:
    xmake run folio
# Run all the tests
test:
    xmake run test

# Format and lint all the code
lint:
    cargo clippy --all-targets --all-features --allow-dirty --fix
    cargo fmt
    pnpm lint:fix

