# Q Business Custom Plugins Monorepo

Welcome to the Q Business Custom Plugins Monorepo! This repository contains a collection of examples demonstrating how to build serverless applications with Rust and AWS Lambda for integration with Q Business. Each example showcases different aspects of developing custom plugins for Q Business, leveraging AWS services like DynamoDB, API Gateway, and Lambda.

**Disclaimer:** Sample code, software libraries, command line tools, proofs of concept, templates,
or other related technology are provided as AWS Content or Third-Party Content under the AWS Customer Agreement,
or the relevant written agreement between you and AWS (whichever applies). You should not use this AWS Content or
Third-Party Content in your production accounts, or on production or other critical data. You are responsible for testing,
securing, and optimizing the AWS Content or Third-Party Content, such as sample code, as appropriate for production grade
use based on your specific quality control practices and standards. Deploying AWS Content or Third-Party Content may incur
AWS charges for creating or using AWS chargeable resources, such as running Amazon EC2 instances or using Amazon S3 storage.

## Overview

This monorepo is structured to include multiple examples, each stored in the `examples/` folder. These examples are designed to help you understand and implement custom plugins for Q Business, showcasing best practices and useful patterns.

## Examples

The following table lists the available examples, each linked to its respective directory:

| Example Name          | Description                                                                                                                                                       | Link                                |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| Example 1: Swag Store | A serverless application providing a backend for an internal swag platform, handling products, inventory, categories and several specifics for a customizable UI. | [Swag Store](./examples/swag_store) |

## Services

Each example demonstrates different services:

## Deployment Guide

To deploy any of the examples to AWS Lambda, follow these steps:

1. **Set up AWS Credentials**: Configure your AWS credentials using the AWS CLI or by setting the `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` environment variables.
2. **Use SAM CLI to deploy the backend**: Install the [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html) and use the command `sam build --beta-features && sam deploy` in the example’s directory.
3. **Provide the API output to Q Business**: The SAM CLI will provide an output called `ShoppingCartApi` with a link to the API. Take that link and add it to the `openapi.yaml` file at the top under `servers`, `url`.

Finally, to connect it to Q Business, ensure you have an application setup in Q Business. Follow the guide [setting up a Q Business application](https://docs.aws.amazon.com/amazonq/latest/qbusiness-ug/create-app.html), then [connect a custom plugin through the Q Business administrative dashboard](https://docs.aws.amazon.com/amazonq/latest/qbusiness-ug/custom-plugin.html) with the included `openapi.yaml` file.

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This library is licensed under the MIT-0 License. See the [LICENSE](LICENSE) file.
