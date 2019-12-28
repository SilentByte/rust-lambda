////!
////! Rust Lambda
////! Copyright (c) 2019 SilentByte <https://silentbyte.com/>
////!

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::json_deserializer;

/// This struct represents the incoming lambda request. The body is being passed in by AWS
/// as a string. Since we expect this string to contain JSON data, we will automatically
/// deserialize it to our payload struct. Note that AWS hands us over additional fields,
/// but for the purpose of this demonstration, the body field is sufficient and others are ignored.
#[derive(Deserialize, Debug)]
pub struct LambdaRequest<Data: DeserializeOwned> {
    #[serde(with = "json_deserializer")]
    body: Data,
}

impl<Data: DeserializeOwned> LambdaRequest<Data> {
    pub fn body(&self) -> &Data {
        &self.body
    }
}

/// The outgoing data that is being passed from our lambda to AWS. AWS API Gateway expects at least
/// the following fields. Important to us are status code, headers, and body that make up the
/// resulting HTTP response that is returned to the client.
#[derive(Serialize, Debug)]
pub struct LambdaResponse {
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,

    #[serde(rename = "statusCode")]
    status_code: u16,

    headers: HashMap<String, String>,
    body: String,
}

pub struct LambdaResponseBuilder {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

/// This builder allows us to create a lambda response with a specific status code, a number of
/// headers, and a JSON body that is automatically deserialized.
#[allow(dead_code)]
impl LambdaResponseBuilder {
    pub fn new() -> Self {
        LambdaResponseBuilder {
            status_code: 200,
            headers: HashMap::new(),
            body: "".to_owned(),
        }
    }

    pub fn with_status(mut self, code: u16) -> Self {
        self.status_code = code;
        self
    }

    pub fn with_header<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.headers
            .insert(name.into().to_ascii_lowercase(), value.into());
        self
    }

    pub fn with_json<D: Serialize>(mut self, data: D) -> Self {
        self.headers
            .entry("content-type".to_owned())
            .or_insert_with(|| "application/json".to_owned());

        self.body = serde_json::to_string(&data).unwrap();
        self
    }

    pub fn build(self) -> LambdaResponse {
        LambdaResponse {
            is_base64_encoded: false,
            status_code: self.status_code,
            headers: self.headers,
            body: self.body,
        }
    }
}
