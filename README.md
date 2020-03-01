# in-app-messaging-rust

## Setup Guide
1. Install docker(https://www.docker.com/)
2. Install Rust
3. copy .env.example to .env
4. Install diesel_cli ```cargo install diesel_cli```

## To Run
```bash
docker-compose up
diesel migration run
cargo run
```