use aws_lambda_events::event::sqs::SqsEvent;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use hyper::Request;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_span_events(FmtSpan::NONE)
        .init();

    // One-shot when invoked from SQS
    #[cfg(feature = "sqs_trigger")]
    {
        let func = service_fn(handle_sqs_event);
        lambda_runtime::run(func).await
    }

    // One-shot when invoked from API Gateway
    #[cfg(feature = "api_gateway_trigger")]
    {
        let trace_layer =
            TraceLayer::new_for_http().on_request(|request: &Request<Body>, _: &tracing::Span| {
                tracing::info!(
                    method = %request.method(),
                    uri = %request.uri(),
                    headers = ?request.headers(),
                    message = "begin request!"
                )
            });

        let app = Router::new()
            .route("/", get(handle_root))
            .route("/from_api", get(handle_from_api))
            .layer(trace_layer);

        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);
        lambda_http::run(app).await;
        Ok(())
    }

    // Run a server that listens for requests
    #[cfg(feature = "local_trigger")]
    {
        let trace_layer =
            TraceLayer::new_for_http().on_request(|request: &Request<Body>, _: &tracing::Span| {
                tracing::info!(
                    method = %request.method(),
                    uri = %request.uri(),
                    headers = ?request.headers(),
                    message = "begin request!"
                )
            });

        let app = Router::new()
            .route("/", get(handle_root))
            .route("/from_api", get(handle_from_api))
            .layer(trace_layer);

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
        Ok(())
    }
}

async fn handle_root() -> impl IntoResponse {
    let data = "Hello, Root!";
    (StatusCode::OK, data).into_response()
}

async fn handle_from_api() -> impl IntoResponse {
    let data = "Hello, API Gateway!";
    (StatusCode::OK, data).into_response()
}

async fn handle_sqs_event(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    let sqs_event = event.payload;
    let records = sqs_event.records;
    for record in records {
        tracing::info!("Got record: {:?}", record);
    }
    Ok(())
}
