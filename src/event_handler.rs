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

pub(crate) async fn function_handler(
    event: LambdaEvent<S3Event>,
    client: &Client,
) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;

    let s3_event = payload.records.get(0).ok_or(LambdaError::NoRecord)?;

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

    let image = {
        let object = client
            .get_object()
            .bucket(src_bucket)
            .key(src_key)
            .send()
            .await?;

        let contents = object.body.collect().await?;
        ImageReader::new(Cursor::new(contents.into_bytes()))
            .with_guessed_format()?
            .decode()?
    };

    let mod_image = image.thumbnail(16 * 25, 9 * 25);

    let resized_image = {
        let mut buffer = BytesMut::new().writer();

        let encoder = WebPEncoder::new_lossless(&mut buffer);
        mod_image.write_with_encoder(encoder)?;

        let buffer = buffer.into_inner().freeze();
        ByteStream::from(buffer)
    };

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

    Ok(())
}
