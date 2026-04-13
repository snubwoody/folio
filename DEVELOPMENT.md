# Folio Development

Folio is a Tauri app, the project is split into the backend `crates/`, which uses Rust and the frontend `packages`
which uses Svelte and TypeScript. You will need Rust, Node.js, Pnpm, Tauri CLI installed on your machine.

## Setup

```bash
git clone https://www.github.com/snubwoody/folio
```

The project uses [`just`](https://github.com/casey/just) runner to manage commands from different CLI tools.
You can list all the commands using `just -l`. If you don't have just, or don't want to use it, you could 
use the individual CLIs instead.

Install dependencies

```bash
# Install pnpm dependencies
pnpm install

# Install and build rust dependencies
cargo build
```

Add `DATABASE_URL=sqlite://data.db` to your environment or `.env` file 
(this is for [sqlx](https://github.com/launchbadge/sqlx?tab=readme-ov-file#compile-time-verification)).

## Run the app

Run the app in dev mode:

```bash
just dev
# or
cargo tauri dev
```

In debug mode logs will be saved to the `crates/folio-desktop/logs` folder, which is excluded 
from version control. Please log important actions like updating the user's settings 
or editing transactions.

## Lint & format

In order to have a PR accepted, you need to make sure everything passes our Linters, so make sure to run these 
before submitting.

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

On the frontend there are unit tests, ending in `.test.ts`, and browser tests `.spec.ts`. Running tests in 
browser mode will open a Chromium browser, you may need to install playwright browsers.

```bash
# Install playwright browsers
pnpm playwright install --with-deps

# Run tests in browser mode
just test-browser:
# or
pnpm -F @folio/desktop test:browser
```

### Backend

The backend tests are [rust tests](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html) as usual, 
please make sure to add tests for new functionality. Some tests may 
need to interact with a database, for this, use the `#[sqlx::test]` attribute which creates a new database for 
each test. For example:

```rust
#[sqlx::test]
async fn create_category(pool: SqlitePool) -> crate::Result<()>{
	Ok(())
}
```