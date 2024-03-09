# STAGE 1
FROM rust:1.76 as planner
RUN cargo install cargo-chef
COPY . ./app
WORKDIR /app
RUN cargo chef prepare --recipe-path recipe.json

# STAGE 2
FROM rust:1.76 as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# STAGE 3
FROM rust:1.76 as builder
COPY . /app
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
ENV SQLX_OFFLINE=true
RUN cargo build --release

# STAGE 4
FROM gcr.io/distroless/cc-debian12
ENV timezone="America/Sao_Paulo"
COPY --from=builder /app/target/release/rinha-backend-2 /app/rinha-backend-2
WORKDIR /app
CMD ["./rinha-backend-2"]