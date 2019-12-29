
# Writing AWS Lambda Functions in Rust

[![AWS Lambda Functions in Rust](https://img.shields.io/badge/Rust-AWS%20Lambda-brown.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://travis-ci.org/SilentByte/nameof.svg?branch=master)](https://travis-ci.org/SilentByte/nameof)
[![MIT License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)

This repository contains a working example of how to create serverless [lambda functions](https://aws.amazon.com/lambda/) using Rust that can be deployed onto [Amazon Web Services](https://aws.amazon.com/).

The project is elaborated in more detail on [SilentByte | Writing AWS Lambda Functions in Rust](https://silentbyte.com/writing-aws-lambda-functions-in-rust).


## Building & Deploying the Project

To build this project, it is required to have [Rust/Cargo](https://rustup.rs/), [NodeJS 12](https://nodejs.org/en/download/), and [Docker](https://www.docker.com/) installed. Having an AWS account with properly configured access keys is not mandatory for building the project but required for deployment.

### Steps

1. Clone this repository.

2. Install the [Serverless Framework](https://serverless.com/): `npm install -g serverless`.

3. Within the project directory, run `npm install`.

4. Run the following command to build and test the lambda function. This will compile the Rust project and build a Docker image for execution, so it may take a while to complete.

    ```bash
    serverless invoke local -f lucky_numbers -d \
               '{"body": "{\"name\":\"SilentByte\", \"count\": 10}"}'
    ```

5. To deploy the project onto AWS, simply run `serverless deploy`. If the deployment succeeded, the CLI command will show you the URL of the API endpoint that has been created. You can then test against it using CURL by running:

    ```bash
    curl -X POST \
         -H "Content-Type: application/json" \
         -d '{"name":"SilentByte","count": 10}' \
         'https://wkawb52awb.execute-api.us-east-1.amazonaws.com/dev/lucky_numbers'
    ```

   With a result that should be similar to this JSON object:

   ```json
   {
     "message": "Hi, 'SILENTBYTE'. Your lucky numbers are:",
     "numbers": [8, 23, 26, 21, 32, 24, 8, 40, 34, 2]
   }
   ```


## License

See [LICENSE.txt](LICENSE.txt).
