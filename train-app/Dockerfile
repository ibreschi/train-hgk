FROM rust:1.61

WORKDIR /app
COPY . .

RUN cargo fetch
RUN cargo install diesel_cli

ARG BUILD
ENV BUILD=$BUILD

CMD ["/app/run-entrypoint.sh"]
