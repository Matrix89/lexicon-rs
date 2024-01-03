use serde_json::json;

use crate::error::XrpcError;

#[derive(Debug)]
pub struct XrpcQuery {
    url: String,
    params: Vec<(String, String)>,
    token: Option<String>,
}

impl XrpcQuery {
    pub fn new(url: String) -> Self {
        Self {
            url,
            params: vec![],
            token: None,
        }
    }

    pub fn param(&mut self, name: String, value: String) {
        self.params.push((name, value));
    }

    pub fn token(&mut self, token: &str) {
        self.token = Some(token.to_owned());
    }

    pub async fn execute<Output>(self) -> Result<Output, XrpcError>
    where
        Output: serde::de::DeserializeOwned + Sized,
    {
        let client = reqwest::Client::new();
        let request = client.get(self.url).query(&self.params);

        let request = if let Some(token) = self.token {
            request.header("Authorization", token)
        } else {
            request
        };

        let result = request.send().await?;
        let text = result.text().await?;
        let response = json!(&text);

        if response.get("error").is_some() {
            let error = response["error"].as_str().unwrap();
            let message = response["message"].as_str().unwrap();
            Err(XrpcError::Xrpc {
                error: error.to_string(),
                message: message.to_string(),
            })
        } else {
            let output = serde_json::from_str::<Output>(&text);
            match output {
                Ok(output) => Ok(output),
                Err(e) => Err(XrpcError::Decode(e, text.clone())),
            }
        }
    }

    pub async fn execute_untyped(self) -> Result<serde_json::Value, XrpcError> {
        self.execute::<serde_json::Value>().await
    }
}
