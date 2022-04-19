FROM rust

RUN cargo install cargo-watch

WORKDIR /usr/src/backend-linker

COPY . .

RUN cargo build

EXPOSE 4000

CMD ["cargo run"]