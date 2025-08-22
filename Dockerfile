
FROM rust:1.76 AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app


COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm src/main.rs

COPY ./src ./src

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rust_compiler*
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM gcr.io/distroless/static-debian12

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust_compiler /

EXPOSE 8080

CMD ["/rust_compiler"]