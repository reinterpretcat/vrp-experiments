variable "env_suffix_name" {
  description = "A suffix used to distinguish between different environments"

  type = string
  default = "dev"
}

variable "solver_data_bucket" {
  description = "Name of the bucket used by solver where problem and solutions are stored"
  type = string
  default = "vrp-solver-data"
}

variable "solver_data_bucket_expiration" {
  description = "Amount of days after which objects are deleted in solver's bucket"
  type = number
  default = 3
}

variable solver_batch_instance_types {
  description = "A list of batch instance types used to solve VRP"
  type = list(string)
  default = [
    "t2.micro"]
}
