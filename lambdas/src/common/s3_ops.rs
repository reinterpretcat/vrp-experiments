use crate::common::AppError;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::error::Error;

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
