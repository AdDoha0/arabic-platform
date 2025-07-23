FROM rust:1.88.0-slim-bookworm

WORKDIR /app

RUN cargo install sqlx-cli --no-default-features --features postgres --locked

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY . .

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

EXPOSE 2000
