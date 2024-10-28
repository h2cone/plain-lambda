use aws_config::BehaviorVersion;
use aws_lambda_events::{
    http,
    s3::object_lambda::{GetObjectContext, S3ObjectLambdaEvent},
};
use aws_sdk_s3::config::StalledStreamProtectionConfig;
use aws_sdk_s3::Client;
use aws_smithy_types::byte_stream::ByteStream;
use bytes::Bytes;
use lambda_runtime::{run, service_fn, LambdaEvent};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_ansi(false)
        .init();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .stalled_stream_protection(
            StalledStreamProtectionConfig::enabled()
                .upload_enabled(false)
                .download_enabled(false)
                .grace_period(Duration::from_secs(60))
                .build(),
        )
        .load()
        .await;
    let client = &Client::new(&config);

    let func = service_fn(move |event| async move { handler(event, client).await });
    if let Err(e) = run(func).await {
        tracing::error!("Handler error: {:?}", e);
    }
    Ok(())
}

pub(crate) async fn handler(
    event: LambdaEvent<S3ObjectLambdaEvent>,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Handler started");
    // Context of the GetObject event
    let context: GetObjectContext = event.payload.get_object_context.unwrap();
    let route = context.output_route;
    let token = context.output_token;
    // Presigned URL of the source object
    let s3_url = context.input_s3_url;
    tracing::info!("Preparing to fetch: {}", s3_url);

    // Range header
    let range = event
        .payload
        .user_request
        .headers
        .get(http::header::RANGE)
        .expect("Range header not found");
    tracing::info!("Get range: {:?}", range);

    let clinet = reqwest::Client::new();
    let mut resp = clinet
        .get(s3_url)
        .header(http::header::RANGE, range)
        .send()
        .await?;
    // Take the headers from the response
    let headers = std::mem::take(resp.headers_mut());
    let conent_range = headers
        .get(http::header::CONTENT_RANGE)
        .expect("Content-Range header not found")
        .to_str()
        .expect("Cannot convert content-range value");
    tracing::info!("Get content range: {}", conent_range);

    // Get the full response text, or stream it
    let content = resp.text().await?;
    // e.g. transform the content to uppercase
    let trans_txt = content.to_uppercase();
    let byte_stream = ByteStream::from(Bytes::from(trans_txt));

    // Pass the transformed object to the GetObject operation
    client
        .write_get_object_response()
        .request_route(route)
        .request_token(token)
        .status_code(200)
        .body(byte_stream)
        .content_range(conent_range)
        .send()
        .await?;
    tracing::info!("Handler ended");
    Ok(())
}
