
type TeaId = Vec<u8>;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  pub delegate_node: TeaId,
  pub ref_num: u32,
  pub cap_cid: Vec<u8>,
  pub model_cid: Vec<u8>,
  pub data_cid: Vec<u8>,
  pub payment: u32,
}