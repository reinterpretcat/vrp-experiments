# Prerequisites

```shell script
# openssl issue with rusoto
sudo apt install openssl1.0 libssl1.0.0 libssl1.0-dev
cargo clean
OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu" 
OPENSSL_INCLUDE_DIR="/usr/include/openssl" 
cargo build
```

# Build lambda

For a custom runtime, AWS Lambda looks for an executable called bootstrap in the deployment package zip.
Rename the generated basic executable to bootstrap and add it to a zip archive.

```shell script
# see https://github.com/awslabs/aws-lambda-rust-runtime/issues/17
cargo install --force --git https://github.com/rust-embedded/cross

cross build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/vrp-api ./target/release/bootstrap
#cp ./target/release/vrp-api ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
#cp ./target/release/vrp-api ./target/release/bootstrap
```


# Terraform

Ensure that AWS credentials and default region is set:

```shell script
$ export AWS_ACCESS_KEY_ID="anaccesskey"
$ export AWS_SECRET_ACCESS_KEY="asecretkey"
$ export AWS_DEFAULT_REGION="us-west-2"
```

```shell script
cd terraform
terraform init
terraform plan
```
