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
