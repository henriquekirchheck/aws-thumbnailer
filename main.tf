terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.0"
    }
    random = {
      source = "hashicorp/random"
      version = "~> 3.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

resource "random_id" "bucket_identifier" {
  byte_length = 4
}

locals {
  bucket_prefix = "thumbnailer-bucket_${random_id.bucket_identifier.hex}"
}

resource "aws_s3_bucket" "original_image" {
  bucket = "${local.bucket_prefix}_original"
}

resource "aws_s3_bucket" "resized_image" {
  bucket = "${local.bucket_prefix}_resized"
}

data "aws_iam_policy_document" "thumbnailer_lambda_role_policy_document" {
  statement {
    effect = "Allow"
    actions = [
      "logs:PutLogEvents",
      "logs:CreateLogGroup",
      "logs:CreateLogStream"
    ]
    resources = ["arn:aws:logs:*:*:*"]
  }
  statement {
    effect    = "Allow"
    actions   = ["s3:GetObject"]
    resources = ["arn:aws:s3:::${aws_s3_bucket.original_image.bucket}"]
  }
  statement {
    effect    = "Allow"
    actions   = ["s3:PutObject"]
    resources = ["arn:aws:s3:::${aws_s3_bucket.resized_image.bucket}"]
  }
}

resource "aws_iam_policy" "thumbnailer_lambda_role_policy" {
  name        = "lambda_policy"
  description = "Policy for lambda to access s3 buckets"
  policy      = data.aws_iam_policy_document.thumbnailer_lambda_role_policy_document.json
}

data "aws_iam_policy_document" "lambda_assume_role_policy" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "thumbnailer_lambda_role" {
  name               = "thumbnailer_lambda_role"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role_policy.json
}

resource "aws_iam_role_policy_attachment" "attach_thumbnail_lambda_role_policy" {
  role = aws_iam_role.thumbnailer_lambda_role.name
  policy_arn = aws_iam_policy.thumbnailer_lambda_role_policy.arn
}

resource "aws_lambda_function" "thumbnail_lambda" {
  function_name = "thumbnailer"
  role = aws_iam_role.thumbnailer_lambda_role.role

  runtime = "provided.al2023"
  handler = "rust.handler"
  architectures = ["arm64"]
  memory_size = 128

  filename = "target/lambda/thumbnailer/bootstrap.zip"
}