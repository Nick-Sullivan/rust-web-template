## Rust Web Template

A basic API template that can run on a server (receiving API requests), and on a Lambda (triggered by API Gateway, or triggered by SQS).

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
curl -X GET http://localhost:3000
```

Via VSCode

You can either use `rust-analyzer: Debug` in VSCode selection, or use `Run` which uses the `launch.json` config in the `.vscode` directory. Note that this does not build, so make sure to build first.

## Creating cloud infrastructure

To create this infrastructure, we'll use the AWS CDK.
To bootstrap (used to create this repo)

```bash
npm install -g aws-cdk
cd infrastructure
cdk init app --language python
python3 -m venv .venv
source .venv/bin/activate
python -m pip install -r requirements.txt
cdk bootstrap
cdk list
```

To initialise:

```bash
source .venv/bin/activate
cdk synth
cdk diff
cdk deploy
```

In the output, take note of the `ApiUrl` and `QueueUrl`

To clean up at the end:

```bash
cdk destroy
```

## API Gateway induced lambda

Use the value of `ApiUrl` from the deployment output.

```bash
export API_URL=https://0sixteekf9.execute-api.ap-southeast-2.amazonaws.com/prod/
curl -X GET $API_URL/from_api
```

## SQS induced lambda

Use the value of `QueueUrl` from the deployment output.

```bash
export QUEUE_URL=https://sqs.ap-southeast-2.amazonaws.com/314077822992/LambdaStack-RequestQueueEA127976-d4ZziG18jqHu
aws sqs send-message --queue-url $QUEUE_URL --message-body "My message"
```
