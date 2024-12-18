terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.56.1"
    }
  }
  backend "s3" {
    bucket = "nicks-terraform-states"
    region = "ap-southeast-2"
    key    = "rust-template.tfstate"
  }
}

provider "aws" {
  region = local.region
  default_tags {
    tags = local.tags
  }
}

data "aws_caller_identity" "identity" {}

locals {
  region           = "eu-west-2"
  prefix           = "RustTemplate"
  prefix_lower     = "rust-template"
  prefix_parameter = "/RustTemplate"
  aws_account_id   = data.aws_caller_identity.identity.account_id
  root_dir         = "${path.root}/.."
  lambda_dir       = "${local.root_dir}/server"
  tags = {
    Project = "Rust Template"
  }
}
