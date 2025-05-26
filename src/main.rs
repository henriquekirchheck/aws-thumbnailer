use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use lambda_runtime::{run, service_fn, tracing, Error};

mod event_handler;
use event_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let shared_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = Client::new(&shared_config);
    let shared_client = &client;

    run(service_fn(
        move |event: lambda_runtime::LambdaEvent<aws_lambda_events::s3::S3Event>| async move {
            function_handler(event, &shared_client).await
        },
    ))
    .await
}
