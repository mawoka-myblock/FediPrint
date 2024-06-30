# Self-Host

## Requirements
- A reverse-proxy (can also be the built-in caddy)
- Docker Compose

## Get started
1. Clone the repo
2. Build the docker images (`docker compose build`)
3. Study the `docker-compose.yml` for config changes.
4. Run `sed -i "s/SECRET/$(openssl rand -hex 32)/g" docker-compose.yml` if you haven't already to set the correct secret.
5. Depoly: `docker compose up -d`
6. Done!

## Extra (Caddy as Reverse proxy)
You can edit the `docker/prod.Caddyfile` and replace the `:8080` with your domain.
Then, uncomment the section in the docker-compose.yml and comment out the other marked section.
