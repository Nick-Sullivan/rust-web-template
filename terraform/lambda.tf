
resource "aws_lambda_function" "api" {
  package_type  = "Image"
  image_uri     = "${aws_ecr_repository.lambda.repository_url}@${data.aws_ecr_image.lambda.id}"
  function_name = "${local.prefix}-API"
  role          = aws_iam_role.lambda_api.arn
  timeout       = 5
  image_config {
    entry_point = ["/api_handler"]
  }
  depends_on = [
    aws_cloudwatch_log_group.api,
    terraform_data.lambda_push,
  ]
}

resource "aws_lambda_function" "sqs" {
  package_type  = "Image"
  image_uri     = "${aws_ecr_repository.lambda.repository_url}@${data.aws_ecr_image.lambda.id}"
  function_name = "${local.prefix}-SQS"
  role          = aws_iam_role.lambda_sqs.arn
  timeout       = 5
  image_config {
    entry_point = ["/sqs_handler"]
  }
  depends_on = [
    aws_cloudwatch_log_group.sqs,
    terraform_data.lambda_push,
  ]
}

resource "aws_iam_role" "lambda_api" {
  name               = "${local.prefix}-API"
  description        = "Allows Lambda run"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

resource "aws_iam_role" "lambda_sqs" {
  name               = "${local.prefix}-SQS"
  description        = "Allows Lambda run"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

resource "aws_iam_role_policy_attachment" "execute_api_lambda" {
  role       = aws_iam_role.lambda_api.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "execute_sqs_lambda" {
  role       = aws_iam_role.lambda_sqs.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy" "read_sqs" {
  name   = "ReadSqs"
  role   = aws_iam_role.lambda_sqs.name
  policy = data.aws_iam_policy_document.read_sqs.json
}

data "aws_iam_policy_document" "lambda_assume_role" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "read_sqs" {
  statement {
    actions = [
      "sqs:ReceiveMessage",
      "sqs:DeleteMessage",
      "sqs:GetQueueAttributes",
    ]
    effect = "Allow"
    resources = [
      aws_sqs_queue.queue.arn,
    ]
  }
}

resource "aws_cloudwatch_log_group" "api" {
  name              = "/aws/lambda/${local.prefix}-API"
  retention_in_days = 90
}

resource "aws_cloudwatch_log_group" "sqs" {
  name              = "/aws/lambda/${local.prefix}-SQS"
  retention_in_days = 90
}
