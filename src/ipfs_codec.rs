pub const OP_DELIVER_SUB_MESSAGE: &'static str = "deliver_sub_message";
pub const OP_PUBSUB_SUB: &'static str = "pubsub_sub";
pub const OP_PUBSUB_PUB: &'static str = "pubsub_pub";
pub const OP_PUBSUB_RANDOM_SUB: &'static str = "pubsub_random_sub";

pub const OP_BLOCK_GET: &'static str = "block_get";
pub const OP_GET: &'static str = "ipfs_get";
pub const OP_BLOCK_GET_ASYNC: &'static str = "block_get_async";
pub const OP_BLOCK_PUT: &'static str = "block_put";
// pub const OP_DAG_GET_DATA: &'static str = "dag_get_data";
pub const OP_DHT_PROV: &'static str = "dht_provide"; 
pub const OP_DHT_FINDPROVS: &'static str = "dht_find_providers"; 
pub const OP_DELIVER_FOUND_PROV: &'static str = "dht_deliver_found_provider";
pub const OP_ID: &'static str = "id";
pub const OP_DELAY_INVOKE: &'static str = "delay_invoke";
pub const OP_FIND_PROV_ERR: &'static str = "find_provider_error";
pub const OP_IS_BLOCK_LOCAL: &'static str = "is_block_local";
pub const OP_OBJ_STAT: &'static str = "block_stat";
pub const OP_PIN_ADD_RECURSIVE: &'static str = "pin_add_recursive";
pub const OP_PIN_ADD_NONRECURSIVE: &'static str = "pin_add_nonrecursive";
pub const OP_IS_PIN: &'static str = "pin_ls_exists";
pub const OP_OBJ_GET: &'static str = "object_get";
pub const OP_OBJ_DATA: &'static str = "object_data";
pub const OP_OBJ_LINKS: &'static str = "object_links";
pub const OP_OBJ_NEW: &'static str = "object_new";
pub const OP_SWARM_PEERS: &'static str = "swarms_peers";

pub const RSA_PUBKEY: &'static str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0NCk1Ed3dEUVlKS29aSWh2Y05\
BUUVCQlFBREt3QXdLQUloQU1KYUlLVkZHbWZMMzNNa3Rvb2ZxclBKUXBic09WT3UNCkQ5Q0lFRnEvcURMRkFnTUJBQUU9\
DQotLS0tLUVORCBQVUJMSUMgS0VZLS0tLS0NCg==";

pub const RSA_PRIKEY: &'static str = "LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQ0KTUlHckFnRUFBaUV\
Bd2xvZ3BVVWFaOHZmY3lTMmloK3FzOGxDbHV3NVU2NFAwSWdRV3Irb01zVUNBd0VBQVFJZw0KS3FjN0NPcHNqQ0ViTGhy\
OWtMeTRhcEJ6WTJUVE5NdlFyZUpwa2VTbUYwRUNFUUR2NDB3NmdyUEpyc0VqSFM5Vw0KYkNhVkFoRUF6MmZmb1hQUWpib\
m1OTkJaaGhxL2NRSVJBTEwwUTVlYWpYaTFMMHQ4cUNXMmhJa0NFUUNMc1kzSg0KSThWTFAvT1NqT1pQVWNLeEFoQk1oal\
hWTWNXMnh6NGFqK25sU05CbQ0KLS0tLS1FTkQgUlNBIFBSSVZBVEUgS0VZLS0tLS0NCg==";

pub const ED_PUBKEY: &'static str = "df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b";
pub const ED_PRIKEY: &'static str = "5579a3c220146f0caaab49b884de505098b89326970b929d781cf4a65445a917df38cb4f12479041c8e8d238109ef2a150b017f382206e24fee932e637c2db7b";


/// Describes an HTTP request
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubSubResponse {
	pub from: Option<String>,
	pub data: Option<String>,
	pub seqno: Option<String>,
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
pub struct ActorCallBackRequest{
	pub operation: String,
	pub payload: Vec<u8>,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubPubRequest{
	pub topic: String,//base64 string of ref_num
	pub payload: String,//base64 encoded message payload
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPutRequest {
	pub data: Vec<u8>,
	pub pin: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPutResponse{
	pub key: String,
	pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindProvidersRequest{
	pub deployment_id: String,
	pub uuid: String,
	pub context: FindProvidersContext,
	pub finding_mode: FindingMode,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FindingMode{
	AsMuchAsPossible,
	FirstComeThenDone,

}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindProvidersContext{
	pub nats_subject: String,
	pub retry_remain_times: u32,
	pub delay_secs: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindProvidersResponseItem{
	pub id: String,
	pub addrs: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindProvidersResponse{
	pub items: Vec<FindProvidersResponseItem>,
	pub deployment_id: String, 
	pub attachment: Option<Vec<u8>>,
	pub uuid: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DhtProvideRequest {
	DeploymentId(String),
	PinnerPubKey(Vec<u8>),
}

// #[derive(Debug, PartialEq, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RetryFindProviderRequest {
// 	pub subject: String,
// 	pub delay_req: FindProvidersRequest, 
// 	pub delay_secs: u64,
// }

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FindProviderInvokeError {
	ResponseCountLimitHit(u32),
	ResponseCountLimitHitAndRetry(FindProvidersRequest),
	Others(String),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsBlockLocalResponse{
	pub result: bool,
	pub error: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectGetResponse{
	pub data: String,
	pub links: Vec<IpfsHeader>,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpfsHeader{
	pub name: String,
	pub hash: String,
	pub size: u64,
	pub typ: Option<String>,
}
