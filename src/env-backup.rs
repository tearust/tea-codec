use std::time::SystemTime;

pub const OP_GET_VAR: &str = "GetEnvVar";
pub const OP_GET_SYSTEM_TIME: &str = "GetSystemTime";
/// A request to get a single value from the K/V store
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequest {
    pub key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse {
    /// The value returned from the data store
    pub value: String,
    /// Indicates whether the key existed
    pub exists: bool,
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemTimeRequest {
    pub param: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSystemTimeResponse {
    /// The value returned from the data store
    pub value: SystemTime,
}