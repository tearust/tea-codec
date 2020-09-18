pub const OPT_INIT: &'static str = "opt_ra_init";


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitPayload {
  pub cid_manifest: String,
  pub manifest_str: String,
}
