use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use hyper::Request;
use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();

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
        .route("/hello", get(handle_hello))
        .layer(trace_layer);

    // One-shot when invoked from API Gateway
    #[cfg(feature = "api_gateway_trigger")]
    {
        let lambda_app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);
        lambda_http::run(lambda_app).await?;
    }

    // Run a server that listens for requests for local development
    #[cfg(feature = "local_trigger")]
    {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    Ok(())
}

async fn handle_hello() -> impl IntoResponse {
    let data = "Hello!";
    (StatusCode::OK, data).into_response()
}
