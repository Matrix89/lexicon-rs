#[derive(Debug)]
pub enum XrpcError {
    Transport(reqwest::Error),
    Decode(serde_json::Error, String),
    Xrpc { error: String, message: String },
}

impl From<reqwest::Error> for XrpcError {
    fn from(error: reqwest::Error) -> Self {
        Self::Transport(error)
    }
}
