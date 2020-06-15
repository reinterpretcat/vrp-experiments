use crate::common::AppError;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use std::error::Error;
use std::io::Read;

/// Uploads string data to s3 bucket using parameters specified.
pub async fn upload_to_s3(
    region: Region,
    bucket: String,
    key: String,
    data: String,
) -> Result<(), AppError> {
    S3Client::new(region)
        .put_object(PutObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            body: Some(data.into_bytes().into()),
            ..Default::default()
        })
        .await
        .map_err(|err| AppError {
            code: "".to_string(),
            message: format!("cannot upload to s3: '{}/{}'", bucket, key),
            details: err
                .source()
                .map(|err_src| format!("{}", err_src))
                .unwrap_or_else(|| "unknown".to_string()),
        })
        .map(|_| ())
}

/// Downloads string data from s3 bucket.
pub async fn download_from_s3(
    region: Region,
    bucket: String,
    key: String,
) -> Result<String, AppError> {
    let mut object = S3Client::new(region)
        .get_object(GetObjectRequest {
            bucket: bucket.to_string(),
            key: key.clone(),
            ..Default::default()
        })
        .await
        .map_err(|err| AppError {
            code: "".to_string(),
            message: format!("cannot download from s3: '{}/{}'", bucket, key),
            details: err
                .source()
                .map(|err_src| format!("{}", err_src))
                .unwrap_or_else(|| "unknown".to_string()),
        })?;

    let mut buffer = Vec::new();
    object
        .body
        .take()
        .ok_or_else(|| "cannot get body".to_string())
        .and_then(|body| {
            body.into_blocking_read()
                .read_to_end(&mut buffer)
                .map_err(|err| format!("cannot read body: '{}'", err))
        })
        .and_then(|_| {
            String::from_utf8(buffer)
                .map_err(|err| format!("cannot convert utf8 string: '{}'", err))
        })
        .map_err(|details| AppError {
            code: "".to_string(),
            message: "cannot download s3 object".to_string(),
            details: format!("s3 object key '{}', error: {}", key, details),
        })
}
