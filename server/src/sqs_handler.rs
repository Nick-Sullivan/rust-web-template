use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();

    let func = service_fn(handle_sqs_event);
    lambda_runtime::run(func).await
}

async fn handle_sqs_event(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    let sqs_event = event.payload;
    let records = sqs_event.records;
    for record in records {
        tracing::info!("Got record: {:?}", record);
    }
    Ok(())
}
