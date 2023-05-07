FROM rust:latest as builder

# install requirements
RUN rustup target add wasm32-unknown-unknown
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install perseus-cli --locked
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install wasm-bindgen-cli
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install wasm-opt

# copy the app

WORKDIR /usr/datapoints-earth

COPY . .

# deploy the app

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    make deploy

# Runtime image
FROM debian:bullseye-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries
COPY --from=builder /usr/datapoints-earth/pkg /app/

EXPOSE 8080

ENV PERSEUS_HOST "0.0.0.0"
ENV PERSEUS_PORT "8080"

CMD [ "/app/server" ]
