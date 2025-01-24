# Q Business Custom Plugins

This is a serverless application built with Rust and AWS Lambda that provides a
backend for an internal swag platform. It interacts with an AWS DynamoDB table
to handle various operations related to products, categories, collections,
and carts.

The intent is to leverage this API in a natural language through Q Business Custom
Plugins and demonstrate how pre-existing custom applications for enterprise
can be re-imagined through Generative AI and Q business.

**Disclaimer:** Sample code, software libraries, command line tools, proofs of concept, templates,
or other related technology are provided as AWS Content or Third-Party Content under the AWS Customer Agreement,
or the relevant written agreement between you and AWS (whichever applies). You should not use this AWS Content or
Third-Party Content in your production accounts, or on production or other critical data. You are responsible for testing,
securing, and optimizing the AWS Content or Third-Party Content, such as sample code, as appropriate for production grade
use based on your specific quality control practices and standards. Deploying AWS Content or Third-Party Content may incur
AWS charges for creating or using AWS chargeable resources, such as running Amazon EC2 instances or using Amazon S3 storage.

## Architecture

This architecture defines the current state of the application.

![sls-shopping-cart.svg](./docs/images/sls-shopping-cart.svg)

## Services

### Product Service

- `GET /products`: Retrieves a list of all products.
- `GET /product/<product_id>`: Retrieves details of a specific product.
- `GET /category/<category_id>/products`: Retrieves a list of products belonging to a specific category.
- `GET /collection/<collection_handle>`: Retrieves a list of products belonging to a specific collection.

### Category Service

- `GET /categories`: Retrieves a list of all categories.
- `GET /category/<category_id>`: Retrieves details of a specific category.

### Cart Service

- `POST /cart`: Creates a new cart.
- `POST /cart/<cart_id>/add`: Adds a product to the cart.
- `GET /cart/<cart_id>`: Retrieves the contents of a cart.
- `DELETE /cart/<cart_id>/remove/<product_id>`: Removes a product from the cart.

### UI Service

- `GET /menu`: Retrieves the menu structure (categories and collections).
- `GET /pages`: Retrieves a list of all pages.
- `GET /page/<page_id>`: Retrieves details of a specific page.

## Deployment Guide

To deploy this application to AWS Lambda, follow these steps:

1. **Set up AWS Credentials**: Configure your AWS credentials using the AWS CLI or by setting the `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` environment variables.
3. **Use SAM CLI to deploy the backend**: Install the [SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/install-sam-cli.html) use the command `sam build --beta-features && sam deploy`
4. **Provide the API output to the frontend**: The SAM CLI will provide an output called `ShoppingCartApi` with a link, pass that link to the amplify environment variable `API_BASE_URL` allowing your frontend to access the API.

Finally, to connect it to Q Business, ensure you have an application setup in Q Business,
follow the guide [setting up a Q Business application](),then,
[connect a custom plugin through the Q Business administrative dashboard]()
with the included [openapi.yaml](./openapi.yaml) file.

Now, you're ready to experience a taste of the future of conversational enterprise
applications.
