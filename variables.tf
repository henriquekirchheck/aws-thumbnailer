variable "aws_region" {
  description = "AWS Region for all resources."
  type        = string
  default     = "sa-east-1"
  validation {
    condition     = var.aws_region == "us-east-2" || var.aws_region == "us-east-1" || var.aws_region == "us-west-1" || var.aws_region == "us-west-2" || var.aws_region == "af-south-1" || var.aws_region == "ap-east-1" || var.aws_region == "ap-south-2" || var.aws_region == "ap-southeast-3" || var.aws_region == "ap-southeast-5" || var.aws_region == "ap-southeast-4" || var.aws_region == "ap-south-1" || var.aws_region == "ap-northeast-3" || var.aws_region == "ap-northeast-2" || var.aws_region == "ap-southeast-1" || var.aws_region == "ap-southeast-2" || var.aws_region == "ap-southeast-7" || var.aws_region == "ap-northeast-1" || var.aws_region == "ca-central-1" || var.aws_region == "ca-west-1" || var.aws_region == "eu-central-1" || var.aws_region == "eu-west-1" || var.aws_region == "eu-west-2" || var.aws_region == "eu-south-1" || var.aws_region == "eu-west-3" || var.aws_region == "eu-south-2" || var.aws_region == "eu-north-1" || var.aws_region == "eu-central-2" || var.aws_region == "il-central-1" || var.aws_region == "mx-central-1" || var.aws_region == "me-south-1" || var.aws_region == "me-central-1" || var.aws_region == "sa-east-1"
    error_message = "value"
  }
}
