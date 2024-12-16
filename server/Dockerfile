FROM rust:1.83 AS builder
WORKDIR /app

# Install system dependencies
RUN apt-get update \
    && apt-get install -y musl-tools musl libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add x86_64-unknown-linux-musl

# Build the project
COPY . .
RUN cargo build \
        --features is_lambda \
        --release \
        --bin server \
        --target x86_64-unknown-linux-musl \
    && strip target/x86_64-unknown-linux-musl/release/server
    
# Create an AWS Lambda compatible image
FROM public.ecr.aws/lambda/provided:al2
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /server

ENV AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH=true

ENTRYPOINT ["/server"]
