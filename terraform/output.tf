output "api_gateway_url" {
  description = "URL for invoking API Gateway."
  value       = aws_api_gateway_stage.gateway.invoke_url
}

output "sqs_url" {
  description = "URL for adding SQS messages."
  value       = aws_sqs_queue.queue.url
}
