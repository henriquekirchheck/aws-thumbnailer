use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::{primitives::ByteStream, Client};
use bytes::{BufMut, BytesMut};
use image::{codecs::webp::WebPEncoder, ImageReader};
use lambda_runtime::{
    tracing::{self},
    Error, LambdaEvent,
};
use std::io::Cursor;
use thiserror::Error;

#[derive(Debug, Error)]
enum LambdaError {
    #[error("No record in event")]
    NoRecord,
    #[error("No bucket info in S3Entity: {0}")]
    NoBucketInfo(&'static str),
    #[error("Invalid source file key: {0}")]
    InvalidFileKey(String),
}

pub(crate) async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;

    let s3_event = payload.records.get(0).ok_or(LambdaError::NoRecord)?;

    let region_provider = RegionProviderChain::default_provider().or_else("sa-east-1");
    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&shared_config);

    let (src_bucket, src_key) = (
        s3_event
            .s3
            .bucket
            .name
            .as_ref()
            .ok_or(LambdaError::NoBucketInfo("name"))?,
        s3_event
            .s3
            .object
            .key
            .as_ref()
            .ok_or(LambdaError::NoBucketInfo("key"))?,
    );

    tracing::info!("Source Bucket: {:?}", src_bucket);
    tracing::info!("Source Key: {:?}", src_key);

    let resized_image = transform_image(
        client
            .get_object()
            .bucket(src_bucket)
            .key(src_key)
            .send()
            .await?
            .body,
    )
    .await?;

    let (dst_bucket, dst_key) = (
        src_bucket.replace("original", "resized"),
        format!(
            "{}.webp",
            src_key
                .rsplit_once('.')
                .ok_or(LambdaError::InvalidFileKey(src_key.to_owned()))?
                .0
        ),
    );
    tracing::info!("Destination Bucket: {:?}", dst_bucket);
    tracing::info!("Destination Key: {:?}", dst_key);

    client
        .put_object()
        .bucket(dst_bucket)
        .key(dst_key)
        .content_type("image/webp")
        .body(resized_image)
        .send()
        .await?;

    tracing::info!("Sucessufuly added modified image to bucket");

    Ok(())
}

async fn transform_image(src: ByteStream) -> Result<ByteStream, Error> {
    tracing::info!("Got original image");
    let bytes = src.collect().await?.into_bytes();
    tracing::info!("Got bytes");
    let cursor = Cursor::new(bytes);
    tracing::info!("Created Cursor");
    let reader = ImageReader::new(cursor);
    tracing::info!("Created Image Reader");
    let reader = reader.with_guessed_format()?;
    tracing::info!("Guessed File Format");
    let original = reader.decode()?;
    tracing::info!("Decoded original image");

    let modified = original.thumbnail(original.width() / 2, original.height() / 2);
    tracing::info!("Applied modifications to image");

    let mut buffer = BytesMut::new().writer();
    let encoder = WebPEncoder::new_lossless(&mut buffer);
    modified.write_with_encoder(encoder)?;
    let buffer = buffer.into_inner().freeze();
    tracing::info!("Wrote modifications to Buffer");

    Ok(ByteStream::from(buffer))
}

#[cfg(test)]
mod tests {
    use aws_sdk_s3::primitives::ByteStream;
    use lambda_runtime::Error;
    use tokio::io::AsyncWriteExt;

    use crate::event_handler::transform_image;

    #[tokio::test]
    async fn test_transform_image() -> Result<(), Error> {
        let file = ByteStream::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/test/test.png")).await?;
        let tranformed = transform_image(file).await?;
        let mut result = tokio::fs::File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/test/test.webp")).await?;
        result
            .write_all(&tranformed.collect().await?.into_bytes())
            .await?;

        Ok(())
    }
}
