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

If you decided to use different AWS region, adjust url in tests scripts according to
[aws documentation](https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-call-api.html).
Then determine api gateway id which you can find by looking into AWS console or terraform output.

To post the test problem, run the following command:

```shell script
./scripts/test_submit_problem.sh api_gateway_id
```

Replace `api_gateway_id` with real one.

If everything is ok, you will get json response like:

```json
{
  "id": "ecc9c997-5063-4eb3-8099-1c3c1743b4f1"
}
```

You can get solution or its processing status by running:

```shell script
./scripts/test_poll_solution.sh api_gateway_id ecc9c997-5063-4eb3-8099-1c3c1743b4f1
```

Result is one of the following responses:

- `200 OK`: solution is calculated and returned in response body
- `204 No content`: problem is accepted, but solution is not yet calculated
- `404 Not found`: no problem is found
- `409 Conflict`: an error is occurred while calculating solution. Response body contains additional information


# Further improvements

- limit resources accessed by each service
- add openapi spec for API Gateway
- add unit tests