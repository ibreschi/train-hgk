FROM rust:1.61

WORKDIR /app
COPY ./ /app

RUN cargo install --path .

CMD ["train-app"]
