variable "env_suffix_name" {
  type = string
  default = "dev"
}

variable "solver_data_bucket" {
  type = string
  default = "vrp-solver-data-dev"
}

variable "solver_data_bucket_expiration" {
  description = "Amount of days after which objects are deleted in solver's bucket"

  type = number
  default = 3
}
