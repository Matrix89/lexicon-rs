use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum XrpcError {
    Transport(reqwest::Error),
    Websocket(tungstenite::Error),
    Decode(serde_json::Error, String),
    Xrpc { error: String, message: String },
}

impl From<reqwest::Error> for XrpcError {
    fn from(error: reqwest::Error) -> Self {
        Self::Transport(error)
    }
}

impl From<tungstenite::Error> for XrpcError {
    fn from(error: tungstenite::Error) -> Self {
        Self::Websocket(error)
    }
}
