
resource "aws_sqs_queue" "queue" {
  name                      = "${local.prefix}-Queue"
  message_retention_seconds = 6 * 60 * 60
}

resource "aws_lambda_event_source_mapping" "lambda" {
  event_source_arn = aws_sqs_queue.queue.arn
  function_name    = aws_lambda_function.sqs.function_name
  batch_size       = 1
}
