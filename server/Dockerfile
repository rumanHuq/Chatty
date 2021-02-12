# context is <PROJECT_ROOT>
FROM lukemathwalker/cargo-chef as planner
WORKDIR /rust-chat
COPY Cargo.lock Cargo.toml ./
COPY server/ ./server
COPY common/ ./common
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR /rust-chat
COPY --from=planner /rust-chat/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.49.0 as builder
WORKDIR /rust-chat
EXPOSE 50051
RUN rustup component add rustfmt
RUN rustup set profile minimal
COPY --from=cacher /rust-chat target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY Cargo.lock Cargo.toml ./
COPY server ./server
COPY common ./common
RUN cargo build -p server --release --bin server

FROM rust:1.49.0 as dev
WORKDIR /rust-chat
EXPOSE 50051
RUN cargo install cargo-watch sqlx-cli
RUN rustup component add rustfmt
RUN rustup set profile minimal
COPY --from=cacher /rust-chat target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY ./migrations /rust-chat/migrations
ENTRYPOINT ["sh","/rust-chat/migrations/run.sh"]
CMD ["cargo", "watch", "-x", "run",  "-p", "server" ]

FROM rust:1.49.0 as runtime
WORKDIR /rust-chat
EXPOSE 50051
COPY --from=builder /rust-chat/target/release/server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/server"]

