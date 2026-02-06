set windows-shell := ["powershell"]

# Run the desktop app
run:
    poetry run py folio/main.py

# Build qrc resources
gen-resources:
    poetry run pyside6-rcc folio/resources.qrc -o folio/rc_resources.py

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

lint-web:
    pnpm eslint . --max-warnings 0
#    pnpm stylelint **/*.{css,svelte} --max-warnings 0

# Format code
[windows]
fmt:
    cargo fmt
    qmlformat -i (Get-ChildItem crates/folio-qt/ui/*.qml).FullName

[unix]
fmt:
    cargo fmt
    qmlformat -i qml/*.qml

# Check formatting
[windows]
fmt-check:
    cargo fmt --all --check
    qmlformat (Get-ChildItem crates/folio-qt/ui/*.qml).FullName

[unix]
fmt-check:
    cargo fmt --all --check
    qmlformat qml/*.qml


# Switch to release
bundle:
    cp target/debug/folio-qt.exe build/folio-qt.exe
    windeployqt build/folio-qt.exe
#    mkdir build
#    cargo packager
