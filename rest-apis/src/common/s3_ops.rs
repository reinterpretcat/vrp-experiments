use crate::common::AppError;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use std::error::Error;
use tokio::runtime::{Builder, Runtime};

pub struct SyncS3 {
    region: Region,
    runtime: Runtime,
}

impl SyncS3 {
    pub fn new(region: Region) -> Result<Self, AppError> {
        Ok(SyncS3 {
            region,
            runtime: Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .map_err(|err| AppError {
                    code: "".to_string(),
                    message: "cannot create async runtime".to_string(),
                    details: format!("{}", err),
                })?,
        })
    }

    pub fn upload_to_s3(&mut self, bucket: &str, key: &str, data: String) -> Result<(), AppError> {
        let region = self.region.clone();
        self.runtime.block_on(async move {
            upload_to_s3_with_client(&S3Client::new(region), bucket, key, data).await?;
            Ok(())
        })
    }
}

/// Uploads string data to s3 bucket using parameters specified.
pub async fn upload_to_s3_with_client(
    client: &S3Client,
    bucket: &str,
    key: &str,
    data: String,
) -> Result<(), AppError> {
    client
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
