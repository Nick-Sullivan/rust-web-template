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
        --no-default-features \
        --features api_gateway_trigger \
        --release \
        --bin api_handler \
        --bin sqs_handler \
        --target x86_64-unknown-linux-musl \
    && strip target/x86_64-unknown-linux-musl/release/api_handler \
    && strip target/x86_64-unknown-linux-musl/release/sqs_handler
    
# Create an AWS Lambda compatible image
FROM public.ecr.aws/lambda/provided:al2
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api_handler /api_handler
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sqs_handler /sqs_handler

ENV AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH=true

ENTRYPOINT ["/api_handler"]
