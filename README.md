## Rust Web Template

A basic Rust API template that can run on a server (receiving API requests), and on a Lambda (triggered by API Gateway, or triggered by SQS).

It uses `axum` as the web application framework, with `axum-aws-lambda` to enable use on a lambda.

## Dependencies

Rust is installed and managed by `rustup`.

```bash
rustup update
rustc --version
```

I use VSCode with the extensions `rust-analyzer` for syntax highlighting, linting, formatting and `vadimcn.vscode-lldb` for debugging in a linux environment.

## Build

```bash
cd server
cargo build
```

## Running locally as a server

Via command line.

```bash
cd server
cargo run
curl -X GET http://localhost:3000/hello
```

Via VSCode

You can either use `rust-analyzer: Debug` in VSCode selection, or use `Run` which uses the `launch.json` config in the `.vscode` directory. Note that this does not build, so make sure to build first.

## Creating cloud infrastructure

To create this infrastructure, we use terraform.

```
cd terraform
terraform init
terraform apply
```

In the output, take note of the `api_gateway_url` and `sqs_url`

To clean up at the end:

```bash
terraform destroy
```

## API Gateway induced lambda

```bash
export API_GATEWAY_URL=<api_gateway_url>
curl -X GET $API_GATEWAY_URL/hello
```

It should respond with "Hello!".

## SQS induced lambda

```bash
export QUEUE_URL=<sqs_url>
aws sqs send-message --queue-url $QUEUE_URL --message-body "My message" --region eu-west-2
```

Go to cloudwatch logs to see the invocation, it will be in `eu-west-2` log group `/aws/lambda/RustTemplate-SQS`.
