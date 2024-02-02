use aws_config::BehaviorVersion;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body::Text;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // Initialize the AWS SDK for Rust
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    run(service_fn(
        |request: LambdaEvent<ApiGatewayProxyRequest>| function_handler(request),
    ))
    .await
}

async fn function_handler(
    evt: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let default_name = String::from("world");
    let who = evt
        .payload
        .path_parameters
        .get("name")
        .unwrap_or(&default_name);
    let body = format!("Hello from {}", who);

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Text(body)),
        is_base64_encoded: false,
    })
}
