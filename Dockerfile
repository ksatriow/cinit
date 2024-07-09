FROM rust:latest

WORKDIR /usr/src/cicd_initializer

COPY . .

RUN cargo build --release

RUN cargo test

RUN cp target/release/cinit /usr/local/bin/cinit

CMD ["cinit", "--version"]
