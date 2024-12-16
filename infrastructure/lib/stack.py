from aws_cdk import (
    CfnOutput,
    Duration,
    Stack,
    aws_apigateway,
    aws_iam,
    aws_lambda,
    aws_lambda_event_sources,
    aws_logs,
    aws_sqs,
)
from constructs import Construct


class RustTemplate(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)
        server_dir = "../server"

        # This will create its own ECR repository and take care of the image.
        # It won't clean up old images, doing that requires a more complex set up.
        function_name = "RustTemplateLambda"
        func = aws_lambda.DockerImageFunction(
            self,
            function_name,
            code=aws_lambda.DockerImageCode.from_image_asset(server_dir),
            timeout=Duration.seconds(10),
        )

        log_group = aws_logs.LogGroup(
            self,
            "RustTemplateLogGroup",
            log_group_name=f"/aws/lambda/{function_name}",
            retention=aws_logs.RetentionDays.THREE_MONTHS,
        )
        log_policy_statement = aws_iam.PolicyStatement(
            actions=["logs:CreateLogStream", "logs:PutLogEvents"],
            resources=[log_group.log_group_arn],
        )
        func.add_to_role_policy(log_policy_statement)

        api = aws_apigateway.LambdaRestApi(
            self, "RustTemplateApi", handler=func, proxy=False
        )
        items = api.root.add_resource("from_api")
        items.add_method("GET")
        items.add_method("POST")

        CfnOutput(self, "ApiUrl", value=api.url)

        queue = aws_sqs.Queue(
            self,
            "RequestQueue",
            visibility_timeout=Duration.seconds(300),
        )
        event_source = aws_lambda_event_sources.SqsEventSource(queue)
        func.add_event_source(event_source)

        CfnOutput(self, "QueueUrl", value=queue.queue_url)
