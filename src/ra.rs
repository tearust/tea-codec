pub const OP_INIT: &str = "opt_ra_init";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitPayload {
	pub manifest_cid: String,
	pub manifest_str: String,
}
