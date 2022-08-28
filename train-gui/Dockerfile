FROM rust:1.61

WORKDIR /app
COPY . .

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo fetch

ARG BUILD
ENV BUILD=$BUILD

CMD ["/app/run-entrypoint.sh"]
