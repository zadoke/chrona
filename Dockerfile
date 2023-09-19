FROM rust:latest

WORKDIR /usr/src/chrona
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]