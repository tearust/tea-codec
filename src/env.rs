pub const OP_GET_VAR: &str = "GetEnvVar";
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