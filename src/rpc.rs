//! JSON-RPC 2.0 request/response types and parsing for the vsock channel to the host.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: RequestId,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

#[derive(Debug)]
pub enum RpcParseError {
    InvalidJson(serde_json::Error),
}

pub fn parse_request(raw: &str) -> Result<RpcRequest, RpcParseError> {
    serde_json::from_str(raw).map_err(RpcParseError::InvalidJson)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_request() {
        let raw = r#"{"jsonrpc":"2.0","id":1,"method":"clipboard.sync","params":{"text":"hi"}}"#;
        let req = parse_request(raw).expect("valid request should parse");
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, RequestId::Number(1));
        assert_eq!(req.method, "clipboard.sync");
        assert_eq!(req.params, Some(serde_json::json!({"text": "hi"})));
    }

    #[test]
    fn rejects_malformed_request() {
        let raw = r#"{"jsonrpc":"2.0","id":1"#;
        let err = parse_request(raw).expect_err("malformed request should not parse");
        assert!(matches!(err, RpcParseError::InvalidJson(_)));
    }
}
