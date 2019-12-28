////!
////! Rust Lambda
////! Copyright (c) 2019 SilentByte <https://silentbyte.com/>
////!

mod lambda_gateway;

use lambda_runtime::{error::HandlerError, lambda, Context};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use lambda_gateway::{LambdaRequest, LambdaResponse, LambdaResponseBuilder};

/// This is the JSON payload we expect to be passed to us by the client accessing our lambda.
#[derive(Deserialize, Debug)]
struct InputPayload {
    name: String,
    count: u32,
}

/// This is the JSON payload we will return back to the client if the request was successful.
#[derive(Serialize, Debug)]
struct OutputPayload {
    message: String,
    numbers: Vec<u8>,
}

fn get_name_or_default(name: &str) -> &str {
    if name.trim().is_empty() {
        "Anonymous"
    } else {
        name
    }
}

/// This is where most of the work of our Rust lambda is done. Once an incoming request arrives
/// at our lambda, this function will be invoked along with the request payload we have defined
/// earlier. It expects a LambdaResponse containing our response payload. In this example,
/// we simply return a message to the client and generate a bunch of lucky numbers.
fn lambda_handler(
    e: LambdaRequest<InputPayload>,
    _c: Context,
) -> Result<LambdaResponse, HandlerError> {
    let payload = e.body();
    let name = get_name_or_default(&payload.name);
    let count = std::cmp::min(std::cmp::max(2, payload.count), 20);

    let response = LambdaResponseBuilder::new()
        .with_status(200)
        .with_json(OutputPayload {
            message: format!("Hi, {}. Your lucky numbers are:", name),
            numbers: (1..=count).map(|_| thread_rng().gen_range(1, 42)).collect(),
        })
        .build();

    Ok(response)
}

/// The main function registers our lambda handler which will be called for every incoming request.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(lambda_handler);
    Ok(())
}
