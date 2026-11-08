FROM node:23-bookworm-slim AS web-build
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY web /web
WORKDIR /web
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM lukemathwalker/cargo-chef:0.1.78-rust-1-slim-bookworm AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends sqlite3 && rm -rf /var/lib/apt/lists/*
# DB only exists so sqlx's query! macros can check queries at compile time
ENV DATABASE_URL=sqlite:///my-feed-data/db
RUN mkdir -p /my-feed-data
COPY migrations migrations
RUN for f in $(printf "%s\n" migrations/*.sql | sort -V); do sqlite3 /my-feed-data/db < "$f"; done

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release -p my-feed --recipe-path recipe.json
# Build application
COPY migrations migrations
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release -p my-feed

FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=web-build /web/dist /my-feed-web
COPY --from=builder /app/target/release/my-feed /my-feed
ENV LISTEN_ADDR=0.0.0.0:80
ENTRYPOINT ["/my-feed"]
