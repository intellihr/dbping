FROM rust:1.37.0

RUN rustup component add rustfmt
WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

CMD ["dbwait"]
