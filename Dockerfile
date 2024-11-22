ARG DATABASE_URL
ARG PORT

FROM rust:latest as rust-builder
WORKDIR /usr/src/iett
COPY . .

ENV SQLX_OFFLINE=true 
RUN cargo build --release

FROM rust:slim
WORKDIR /usr/src/iett
COPY --from=rust-builder /usr/src/iett/target/release/server .

ENV DATABASE_URL=${DATABASE_URL}
ENV PORT=${PORT}

EXPOSE ${PORT}
ENTRYPOINT ["./server"]
