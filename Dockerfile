FROM rust:latest

WORKDIR /usr/src/cptrainbot-backend
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]