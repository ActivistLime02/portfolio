FROM rust:slim AS build

WORKDIR /app
COPY . .

RUN apt update \
    && apt install -y --no-install-recommends \
        npm \
    && rm -rf /var/lib/apt/lists/* /var/cache/apt/* \
    && apt clean

RUN npm install

RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    ls target/release; \
    objcopy --compress-debug-sections target/release/portfolio ./main


FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/main ./

COPY --from=build /app/public ./public
COPY --from=build /app/templates ./templates

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

CMD ./main