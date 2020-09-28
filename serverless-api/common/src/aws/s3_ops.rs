use crate::models::{AppError, Context};
use bytes::BytesMut;
use futures::TryStreamExt;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use std::error::Error;

/// Uploads string data to s3 bucket using parameters specified.
pub async fn upload_to_s3(ctx: &Context, key: &str, data: String) -> Result<(), AppError> {
    S3Client::new(ctx.region.clone())
        .put_object(PutObjectRequest {
            bucket: ctx.bucket.clone(),
            key: key.to_string(),
            body: Some(data.into_bytes().into()),
            ..Default::default()
        })
        .await
        .map_err(|err| AppError {
            message: "cannot upload to s3".to_string(),
            details: format!(
                "bucket: '{}', key: '{}', more details: '{}'",
                ctx.bucket,
                key,
                err.source()
                    .map(|err_src| format!("{}", err_src))
                    .unwrap_or_else(|| "unknown".to_string())
            ),
        })
        .map(|_| ())
}

/// Downloads string data from s3 bucket.
pub async fn download_from_s3(ctx: &Context, key: &str) -> Result<String, AppError> {
    let download_error = |err: String| AppError {
        message: "cannot download from s3".to_string(),
        details: format!(
            "bucket: '{}', key: '{}', more details: '{}'",
            ctx.bucket, key, err
        ),
    };

    let object = S3Client::new(ctx.region.clone())
        .get_object(GetObjectRequest {
            bucket: ctx.bucket.clone(),
            key: key.to_string(),
            ..Default::default()
        })
        .await
        .map_err(|err| {
            download_error(
                err.source()
                    .map(|err_src| format!("{}", err_src))
                    .unwrap_or_else(|| "unknown".to_string()),
            )
        })?;

    let body = object
        .body
        .ok_or_else(|| download_error("body is empty".to_string()))
        .map(|stream| stream.map_ok(|b| BytesMut::from(&b[..])).try_concat())?
        .await
        .map_err(|err| download_error(format!("cannot get body: '{}'", err)));

    body.and_then(|body| {
        String::from_utf8(body.to_vec())
            .map_err(|err| download_error(format!("cannot convert to utf8 string: '{}'", err)))
    })
}
