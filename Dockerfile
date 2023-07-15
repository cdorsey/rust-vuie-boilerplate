ARG RUST_VERSION=1.70.0
ARG NODE_VERSION=20

FROM rust:${RUST_VERSION}-alpine as rust

WORKDIR /usr/src/app

RUN apk add --no-cache musl-dev

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./server ./server/
COPY ./xtask ./xtask/

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
    cargo build --package server --release --locked
EOF

FROM node:${NODE_VERSION}-alpine as node

WORKDIR /usr/src/app

RUN corepack enable

COPY ./client/package.json ./client/pnpm-lock.yaml ./

RUN --mount=type=cache,target=/usr/local/share/.cache/pnpm <<EOF
    pnpm install --prod --frozen-lockfile
EOF

COPY ./client ./

RUN --mount=type=cache,target=/usr/local/share/.cache/pnpm <<EOF
    pnpm run build
EOF

FROM alpine:3.18 as runtime

ENV ASSETS_DIR=/var/www/assets

COPY --from=rust /usr/src/app/target/release/server /usr/local/bin/server
COPY --from=node /usr/src/app/dist ${ASSETS_DIR}

CMD ["server"]
