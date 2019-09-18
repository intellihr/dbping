FROM rust:1.37.0

RUN rustup component add rustfmt && rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

CMD ["dbwait"]
