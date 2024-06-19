FROM rust:1.79 as build-env

WORKDIR /app

COPY Cargo.* .
COPY src ./src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

ENV TODO_DATABASE_STRING="/data/todo-db.db"

COPY --from=build-env /app/target/release/todo /

ENTRYPOINT ["./todo"]
