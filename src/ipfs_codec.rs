pub const OP_DELIVER_SUB_MESSAGE: &'static str = "deliver_sub_message";
pub const OP_PUBSUB_SUB: &'static str = "pubsub_sub";
pub const OP_PUBSUB_PUB: &'static str = "pubsub_pub";
pub const OP_PUBSUB_RANDOM_SUB: &'static str = "pubsub_random_sub";

pub const OP_BLOCK_GET: &'static str = "block_get";
pub const OP_BLOCK_PUT: &'static str = "block_put";
pub const OP_DAG_GET_DATA: &'static str = "dag_get_data";
pub const OP_DHT_PROV: &'static str = "dht_provide"; 
pub const OP_DHT_FINDPROVS: &'static str = "dht_find_providers"; 


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
pub struct PubsubRandomSubRequest{
  pub random: String,
  pub discover: bool,
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubRandomSubResponse{
  pub topic: String,
  pub curse: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubPubRequest{
  pub topic: String,//base64 string of ref_num
  pub payload: String,//base64 encoded message payload
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPutResponse{
  pub key: String,
  pub size: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindProvidersRequest{
  pub deployment_id: String,
  pub callback_func: String,
}