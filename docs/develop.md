# Development

## Tools
- rust and cargo stable (install with [rustup.rs](https://rustup.rs))
- Docker/Podman compose
- [pnpm](https://pnpm.ioo) and Node LTS
- (Stripe account)
- Sqlx-CLI (`cargo install sqlx-cli`)


## Get started
1. Clone the repo
```sh
git clone https://github.com/mawoka-myblock/FediPrint && cd FediPrint
```

2. Run the docker development stack
```sh
docker compose -f docker-compose.dev.yml up -d
```

3. Prepare .env
```sh
cp .env.example .env
```

4. Apply the migrations
```sh
sqlx mig run
```

5. Adjust the .env if desired (not really needed, for stripe, check [stripe.md](/docs/stripe-setup.md))
6. Run the backend in the `fediprint/`-folder: `cargo run --bin app` and the worker: `cargo run --bin worker`
7. Install the frontend dependencies and run it in the `frontend/`-folder: `pnpm i` and `pnpm dev`

To round up, you should now have three terminals and a running docker stack in the backround!

## Commit
Make sure you've got pre-commit installed. If not, run the following command before each commit:
```sh
cd fediprint && cargo fmt && cargo sqlx prepare --workspace
```
