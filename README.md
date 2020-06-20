# Description

The intention of this project is to build VRP REST API using Rust and AWS. It uses another Rust project which implements
a rich VRP solver as a library, you can find it [here](https://github.com/reinterpretcat/vrp).


# Prerequisites

Once source code is pulled from github, make sure that you have the following prerequisites:

- linux compatible environment
- rust
    * install rust toolchain using instructions from [official site](https://www.rust-lang.org/tools/install)
    * install cross compilation tool from [github](https://github.com/rust-embedded/cross) which will be used to build
      code to be in AWS:
      ```shell script
        # see https://github.com/awslabs/aws-lambda-rust-runtime/issues/17
        cargo install --force --git https://github.com/rust-embedded/cross
      ```
- AWS
    * create an account in [aws](https://aws.amazon.com/resources/create-account)
    * create Elastic Container Repository with `vrp-solver` name which will be used to publish docker image
    * set environment variables:
    ```shell script
      export AWS_ACCESS_KEY_ID=your-access-key
      export AWS_SECRET_ACCESS_KEY=your-secret-key
      export AWS_DEFAULT_REGION="eu-west-1" # or another one
    ```
- Terraform
    * install terraform using instructions from [official site](https://learn.hashicorp.com/terraform/getting-started/install.html)
    * create `private.tfvars` file inside `terraform` directory and put the following:
    ```
        vpc_subnet_ids = ["subnet-12340a1b"]
        vpc_security_group_ids = ["sg-12340a2b"]
        batch_container_image = "012345678901.dkr.ecr.eu-west-1.amazonaws.com/vrp-solver"
    ```
    Use proper subnet and security group ids (e.g. default ones) and change `012345678901` to your account id (12 digits).

# Build

Use the following script to build the code:

```shell script
    ./scripts/build.sh
```

It builds rust code and copies build artifacts into `artifacts` folder.

# Deploy

- build a docker image of the VRP solver and publish it in Elastic Container Repository:

```shell script
    ./scripts/deploy.sh
```

- create AWS resources using terraform:

```shell script
  cd terraform
  terraform init
  terraform apply -var-file="private.tfvars"
```

# Test service


If you decided to use different AWS region, adjust tests scripts according to [aws documentation](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-call-api.html).



# Further improvements

- limit resources accessed by each service
- add openapi spec for API Gateway
- add unit tests