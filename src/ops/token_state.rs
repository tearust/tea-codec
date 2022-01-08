/// tokenstate state machine op codes
pub const OP_QUERY_STATE_TSID: &str = "QueryStateTsid";
pub const OP_QUERY_TEA_BALANCE: &str = "QueryTeaBalance";
pub const OP_QUERY_TOKEN_BALANCE: &str = "QueryTokenBalance";
pub const OP_QUERY_AUTH_OPS_BYTES: &str = "QueryAuthOpsBytes";
pub const OP_COMMIT_TXN: &str = "CommitTxn";
pub const OP_TOPUP: &str = "Topup";
pub const OP_WITHDRAW: &str = "Withdraw";
pub const OP_MOVE: &str = "Move";
pub const OP_QUERY_APP_AES: &str = "QueryAppAes";
pub const OP_GEN_APP_AES: &str = "GenAppAes";
pub const OP_GET_APP_AES: &str = "GetAppAes";
pub const OP_GEN_APP_CONSUME_ACCT: &str = "GenAppConsumeAcct";
pub const OP_GET_APP_CONSUME_ACCT: &str = "GetAppConsumeAcct";
pub const OP_DEPOSIT: &str = "Deposit";
pub const OP_REFUND: &str = "Refund";
pub const OP_CONSUME_FROM_DEPOSIT: &str = "ConsumeFromDeposit";
pub const OP_CONSUME_FROM_ACCOUNT: &str = "ConsumeFromAccount";
pub const OP_PAYMENT_FROM_DEPOSIT: &str = "PaymentFromDeposit";
pub const OP_EXPORT_STATE: &str = "ExportState";
pub const OP_IMPORT_STATE: &str = "ImportState";
pub const OP_EXPORT_GLUESQL: &str = "ExportGlueSql";
pub const OP_IMPORT_GLUESQL: &str = "ImportGlueSql";
pub const OP_SET_AUTH_OPS_BYTES: &str = "SetAuthOpsBytes";
pub const OP_SET_USER_LOGIN_SESSION_KEY: &str = "SetUserLoginSessionKey";
pub const OP_GET_USER_LOGIN_SESSION_KEY: &str = "GetUserLoginSessionKey";