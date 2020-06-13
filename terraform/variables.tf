# s3
variable "data_bucket" {
  description = "Name of the bucket used by solver where problem and solutions are stored. Default is `vrp-solver-data`"
  type = string
  default = "vrp-solver-data"
}

variable "data_bucket_expiration" {
  description = "Amount of days after which objects are deleted in solver's bucket. Default is `3`"
  type = number
  default = 3
}

# batch
variable batch_instance_types {
  description = "A list of batch instance types used to solve VRP. Default is `t2.micro`"
  type = list(string)
  default = [
    "t2.micro"]
}

variable max_vcpus {
  description = "The maximum number of EC2 vCPUs that an environment can reach. Default: `1`"
  type = string
  default = 1
}

variable min_vcpus {
  description = "The minimum number of EC2 vCPUs that an environment should maintain. Default: `0`"
  type = string
  default = 0
}

variable "batch_vpc_cidr_block" {
  description = "A CIDR block to be used for batch compute environment vpc. Default: `10.1.0.0/16`"
  type = string
  default = "10.1.0.0/16"
}

variable "batch_vpc_subnet_cidr_block" {
  description = "A CIDR block to be used for batch compute environment subnet. Default: `10.1.1.0/24`"
  type = string
  default = "10.1.1.0/24"
}
