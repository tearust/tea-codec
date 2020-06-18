pub const OP_DELIVER_SUB_MESSAGE: &'static str = "deliver_sub_message";
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