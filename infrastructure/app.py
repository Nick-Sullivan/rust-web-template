#!/usr/bin/env python3
import aws_cdk as cdk
from lib.stack import RustTemplate

app = cdk.App()
cdk.Tags.of(app).add("Project", "Rust template")
cdk.Tags.of(app).add("Environment", "production")
RustTemplate(app, "RustTemplate")
app.synth()
