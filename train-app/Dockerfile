FROM rust:1.61

WORKDIR /app
COPY . .

RUN cargo fetch

ARG BUILD
ENV BUILD=$BUILD

CMD ["/app/run-entrypoint.sh"]
