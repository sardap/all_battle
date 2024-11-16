# -------------- FRONTEND BUILDER ----------------
FROM node:18.7.0 AS frontend_builder
WORKDIR /app
# Seprated for caching
COPY ./frontend/package.json ./frontend/package-lock.json ./
RUN npm install .
COPY ./frontend .
RUN npm run build

# -------------- BACKEND BUILDER ----------------
FROM rust:1.79.0-buster AS backend_builder

RUN mkdir -p /app/src
WORKDIR /app

COPY ./rust-toolchain.toml ./rust-toolchain.toml
COPY ./core ./core
COPY ./web ./web
COPY ./run ./run
COPY ./gen ./gen
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

WORKDIR /app/run

RUN cargo build --release --features web

# -------------- RUNNER ----------------
FROM debian:buster-20230411-slim

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev ca-certificates sqlite3 curl

RUN useradd -ms /bin/bash win

RUN mkdir app
WORKDIR /app

RUN mkdir -p /app/target

COPY --from=frontend_builder /app/dist /app/frontend
COPY --from=backend_builder /app/target/release/all-battle-run /app/target/all-battle-run

RUN mkdir /app/config

RUN chown -R win /app

USER win

EXPOSE 8000

RUN mkdir /app/database
RUN mkdir /app/videos

CMD ["/app/target/all-battle-run", "/app/database/database.sql", "/app/videos/", "/app/frontend/"]