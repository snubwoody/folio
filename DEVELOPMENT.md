
## Prerequisites
You will 
## Setup

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

Run the app in dev mode

```bash
just dev

# or

cargo tauri dev
```


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