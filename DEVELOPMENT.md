
[DATABASE_URL]
## Prerequisites
You will 
## Setup
[Or maybe they'll just fork it?]

```bash
git clone https://www.github.com/snubwoody/folio
```

Because multiple CLI tools are used, this repo uses `just` runner to make typing commands simpler. You can ignore it if you want.

Install dependencies

```bash
# Install pnpm dependencies
pnpm install

# Install and build rust dependencies
cargo build
```
This repo uses `just` to make commands simpler

## Run the app

Run the app in dev mode:

```bash
just dev
# or
cargo tauri dev
```

In debug mode logs will be saved to the `crates/folio-desktop/logs` folder, which is excluded from version control.
## Lint & format

In order to have a PR accepted, you need to make sure everything passes our Linters, so make sure to run these before submitting.


- Frontend:
```shell
pnpm lint # Check lint errors
```

- Backend:
```shell
cargo clippy   # Check linting errors
cargo fmt --check # Check format errors
```

- Format and lint code
```bash
just lint
```

## Testing

### Frontend

On the frontend there are unit tests, ending in `.test.ts`, and browser tests `.spec.ts`.

Running tests in browser mode will open a chromium browser, you may need to install playwright browsers.

```bash
# Install playwright browsers
pnpm playwright install --with-deps

# Run tests in browser mode
just test-browser:
# or
pnpm -F @folio/desktop test:browser
```

### Backend

The backend tests are rust tests as usual, please make sure to add tests for new functionality. Some tests may need to interact with a database, for this, use the `#[sqlx::test]` attribute which creates a new database for each test. For example:

```rust
#[sqlx::test]
async fn create_category(pool: SqlitePool) -> crate::Result<()>{
	Ok(())
}
```