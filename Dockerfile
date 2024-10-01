FROM rust:1.81.0-bookworm as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/
RUN mkdir src
COPY src /app/src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/git_monitoring /app/
COPY --from=public.ecr.aws/awsguru/aws-lambda-adapter:0.8.4 /lambda-adapter /opt/extensions/lambda-adapter
CMD ["/app/git_monitoring"]