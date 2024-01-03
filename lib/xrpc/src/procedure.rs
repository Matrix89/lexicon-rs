use std::collections::HashMap;

use serde_json::json;

use crate::error::XrpcError;

pub struct XrpcProcedure<Input> {
    url: String,
    params: HashMap<String, String>,
    token: Option<String>,
    input: Option<Input>,
}

impl<Input> XrpcProcedure<Input>
where
    Input: serde::ser::Serialize + Sized,
{
    pub fn new(url: String) -> Self {
        Self {
            url,
            params: HashMap::new(),
            token: None,
            input: None,
        }
    }

    pub fn param(mut self, name: String, value: String) -> Self {
        self.params.insert(name, value);
        self
    }

    pub fn input(mut self, input: Input) -> Self {
        self.input = Some(input);
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_owned());
        self
    }

    pub async fn execute<Output>(self) -> Result<Output, XrpcError>
    where
        Output: serde::de::DeserializeOwned + Sized,
    {
        let client = reqwest::Client::new();
        let request = client.post(self.url).query(&self.params);

        let request = if let Some(token) = self.token {
            request.header("Authorization", token)
        } else {
            request
        };

        let request = if let Some(input) = self.input {
            request.json(&input)
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
}
