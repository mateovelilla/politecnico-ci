FROM rust:latest
WORKDIR /api

COPY Cargo.toml .
COPY src src
RUN echo ls
RUN cargo build --release

CMD ["target/release/api"]