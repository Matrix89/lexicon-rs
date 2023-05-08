use std::collections::HashMap;

use futures::StreamExt;
use serde_json::json;
use tokio_tungstenite::connect_async;

use crate::error::XrpcError;

#[derive(Debug)]
pub struct XrpcSubscription {
    url: String,
    params: HashMap<String, String>,
    token: Option<String>,
}

impl XrpcSubscription {
    pub fn new(url: String) -> Self {
        Self {
            url,
            params: HashMap::new(),
            token: None,
        }
    }

    pub fn param(&mut self, name: String, value: String) {
        self.params.insert(name, value);
    }

    pub fn token(&mut self, token: &String) {
        self.token = Some(token.clone());
    }

    pub async fn subscribe(self) -> Result<(), XrpcError> {
        println!("Connecting to {}", self.url);
        let result = connect_async(self.url).await;
        println!("{:?}", result);
        let (mut socket, _response) = result.unwrap();
        while let Some(v) = socket.next().await {
            println!("Received: {:?}", v);
        }

        Ok(())

        /*let client = reqwest::Client::new();
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
            return Err(XrpcError::Xrpc {
                error: error.to_string(),
                message: message.to_string(),
            });
        } else {
            let output = serde_json::from_str::<Output>(&text);
            match output {
                Ok(output) => Ok(output),
                Err(e) => Err(XrpcError::Decode(e, /*text.clone()*/ "".to_string())),
            }
        }*/
    }
}
