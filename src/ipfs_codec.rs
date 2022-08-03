pub const OP_DELIVER_SUB_MESSAGE: &str = "deliver_sub_message";

pub const OP_BLOCK_GET: &str = "block_get";
pub const OP_GET: &str = "ipfs_get";
pub const OP_BLOCK_GET_ASYNC: &str = "block_get_async";
pub const OP_BLOCK_PUT: &str = "block_put";
// pub const OP_DAG_GET_DATA: &str = "dag_get_data";
pub const OP_DHT_PROV: &str = "dht_provide";
pub const OP_DHT_FINDPROVS: &str = "dht_find_providers";
pub const OP_DELIVER_FOUND_PROV: &str = "dht_deliver_found_provider";
pub const OP_ID: &str = "id";
pub const OP_DELAY_INVOKE: &str = "delay_invoke";
pub const OP_FIND_PROV_ERR: &str = "find_provider_error";
pub const OP_IS_BLOCK_LOCAL: &str = "is_block_local";
pub const OP_OBJ_STAT: &str = "block_stat";
pub const OP_PIN_ADD_RECURSIVE: &str = "pin_add_recursive";
pub const OP_PIN_ADD_NONRECURSIVE: &str = "pin_add_nonrecursive";
pub const OP_IS_PIN: &str = "pin_ls_exists";
pub const OP_OBJ_GET: &str = "object_get";
pub const OP_OBJ_DATA: &str = "object_data";
pub const OP_OBJ_LINKS: &str = "object_links";
pub const OP_OBJ_NEW: &str = "object_new";
pub const OP_SWARM_PEERS: &str = "swarms_peers";

pub const RSA_PUBKEY: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0NCk1Ed3dEUVlKS29aSWh2Y05\
BUUVCQlFBREt3QXdLQUloQU1KYUlLVkZHbWZMMzNNa3Rvb2ZxclBKUXBic09WT3UNCkQ5Q0lFRnEvcURMRkFnTUJBQUU9\
DQotLS0tLUVORCBQVUJMSUMgS0VZLS0tLS0NCg==";

pub const RSA_PRIKEY: &str = "LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQ0KTUlHckFnRUFBaUV\
Bd2xvZ3BVVWFaOHZmY3lTMmloK3FzOGxDbHV3NVU2NFAwSWdRV3Irb01zVUNBd0VBQVFJZw0KS3FjN0NPcHNqQ0ViTGhy\
OWtMeTRhcEJ6WTJUVE5NdlFyZUpwa2VTbUYwRUNFUUR2NDB3NmdyUEpyc0VqSFM5Vw0KYkNhVkFoRUF6MmZmb1hQUWpib\
m1OTkJaaGhxL2NRSVJBTEwwUTVlYWpYaTFMMHQ4cUNXMmhJa0NFUUNMc1kzSg0KSThWTFAvT1NqT1pQVWNLeEFoQk1oal\
hWTWNXMnh6NGFqK25sU05CbQ0KLS0tLS1FTkQgUlNBIFBSSVZBVEUgS0VZLS0tLS0NCg==";

pub const ED_PUBKEY: &str = "df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b";
pub const ED_PRIKEY: &str = "5579a3c220146f0caaab49b884de505098b89326970b929d781cf4a65445a917df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubSubRequest {
	pub topic: String,
	pub discover: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorCallBackRequest {
	pub operation: String,
	pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPutRequest {
	pub data: Vec<u8>,
	pub pin: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPutResponse {
	pub key: String,
	pub size: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DhtProvideRequest {
	DeploymentId(String),
	PinnerPubKey(Vec<u8>),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FindProviderInvokeError {
	ResponseCountLimitHit(u32),
	ResponseCountLimitHitAndRetry,
	Others(String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsBlockLocalResponse {
	pub result: bool,
	pub error: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectGetResponse {
	pub data: String,
	pub links: Vec<IpfsHeader>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpfsHeader {
	pub name: String,
	pub hash: String,
	pub size: u64,
	pub typ: Option<String>,
}
