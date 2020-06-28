pub const START: &'static str = "start_task_from_runtime";


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TensorflowParam {
  pub image: Vec<u8>,
}