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
        let body = record.body.clone();
        tracing::info!("Got record: {:?}", record);
        if let Some(msg) = body {
            tracing::info!("Message: {:?}", msg);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lambda_events::event::sqs::{SqsEvent, SqsMessage};
    use lambda_runtime::Context;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn test_handle_sqs_event_writes_logs() {
        let record = SqsMessage {
            message_id: Some("1".to_string()),
            receipt_handle: Some("abc".to_string()),
            body: Some("test message".to_string()),
            ..Default::default()
        };
        let event = LambdaEvent::new(
            SqsEvent {
                records: vec![record],
            },
            Context::default(),
        );
        let result = handle_sqs_event(event).await;
        assert!(result.is_ok());
        assert!(logs_contain("Message: \"test message\""));
    }
}
