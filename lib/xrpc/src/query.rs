use std::collections::HashMap;

use serde_json::json;

use crate::error::XrpcError;

#[derive(Debug)]
pub struct XrpcQuery {
    url: String,
    params: HashMap<String, String>,
    token: Option<String>,
}

impl XrpcQuery {
    pub fn new(url: String) -> Self {
        Self {
            url,
            params: HashMap::new(),
            token: None,
        }
    }

    pub fn param(mut self, name: String, value: String) -> Self {
        self.params.insert(name, value);
        self
    }

    pub fn token(mut self, token: &String) -> Self {
        self.token = Some(token.clone());
        self
    }

    pub fn execute<Output>(self) -> Result<Output, XrpcError>
    where
        Output: serde::de::DeserializeOwned + Sized,
    {
        let client = reqwest::blocking::Client::new();
        let request = client.get(self.url).query(&self.params);

        let request = if let Some(token) = self.token {
            request.header("Authorization", token)
        } else {
            request
        };

        let result = request.send()?;
        let text = result.text()?;
        let response = json!(&text);

        if response.get("error").is_some() {
            let error = response["error"].as_str().unwrap();
            let message = response["message"].as_str().unwrap();
            return Err(XrpcError::Xrpc {
                error: error.to_string(),
                message: message.to_string(),
            });
        } else {
            let output = serde_json::from_str::<Output>(&text);
            match output {
                Ok(output) => Ok(output),
                Err(e) => Err(XrpcError::Decode(e, text.clone())),
            }
        }
    }
}
