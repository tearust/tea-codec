pub const OP_DELIVER_SUB_MESSAGE: &'static str = "deliver_sub_message";
pub const OP_PUBSUB_SUB: &'static str = "pubsub_sub";
pub const OP_BLOCK_GET: &'static str = "block_get";
pub const OP_BLOCK_PUT: &'static str = "block_put";


/// Describes an HTTP request
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubSubResponse {
  pub from: Option<Vec<u8>>,
  pub data: Option<Vec<u8>>,
  pub seqno: Option<Vec<u8>>,
  pub topics: Option<Vec<String>>, 
  pub unrecognized: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubSubRequest{
  pub topic: String,
  pub discover: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubPubRequest{
  pub topic: String,
  pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPutResponse{
  pub key: String,
  pub size: u64,
}