/// tokenstate state machine op codes
pub const OP_QUERY_STATE_TSID: &str = "QueryStateTsid";
pub const OP_QUERY_TEA_BALANCE: &str = "QueryTeaBalance";
pub const OP_QUERY_TEA_DEPOSIT_BALANCE: &str = "QueryTeaDepositBalance";
pub const OP_READ_TEA_BALANCE: &str = "ReadTeaBalance";
pub const OP_QUERY_AUTH_OPS_BYTES_AND_NEW_EXPIRE: &str = "QueryAuthOpsBytes";
pub const OP_QUERY_APP_CONSUME_BALANCE: &str = "QueryAppConsumeBalance";
pub const OP_BURN_CONSUME_BACK_TO_SYSTEM_ACCT: &str = "BurnConsumeUnbalanced";
pub const OP_CHECK_TXN: &str = "CheckTxn";
pub const OP_COMMIT_TXN: &str = "CommitTxn";
pub const OP_TOPUP: &str = "Topup";
pub const OP_WITHDRAW: &str = "Withdraw";
pub const OP_MOVE: &str = "Move";
pub const OP_QUERY_APP_AES: &str = "QueryAppAes";
pub const OP_GEN_APP_AES: &str = "GenAppAes";
pub const OP_GET_APP_AES: &str = "GetAppAes";
pub const OP_GEN_TAPPSTORE_ACCT: &str = "GenTappstoreAcct";
pub const OP_GET_TAPPSTORE_ACCT: &str = "GetTappstoreAcct";
pub const OP_GEN_APP_CONSUME_ACCT: &str = "GenAppConsumeAcct";
pub const OP_GET_APP_CONSUME_ACCT: &str = "GetAppConsumeAcct";
pub const OP_DEPOSIT: &str = "Deposit";
pub const OP_REFUND: &str = "Refund";
pub const OP_SLASH: &str = "Slash";
pub const OP_CONSUME_FROM_DEPOSIT: &str = "ConsumeFromDeposit";
pub const OP_CONSUME_FROM_ACCOUNT: &str = "ConsumeFromAccount";
pub const OP_PAYMENT_FROM_DEPOSIT: &str = "PaymentFromDeposit";
pub const OP_EXPORT_STATE: &str = "ExportState";
pub const OP_IMPORT_STATE: &str = "ImportState";
pub const OP_EXPORT_GLUESQL: &str = "ExportGlueSql";
pub const OP_IMPORT_GLUESQL: &str = "ImportGlueSql";
pub const OP_INIT_GLUESQL: &str = "InitGlueSql";
pub const OP_HAS_DB_INIT: &str = "HasDbInit";
pub const OP_EXEC_GLUECMD: &str = "ExecGlueCmd";
pub const OP_EXEC_GLUEQUERY: &str = "ExecGlueQuery";
pub const OP_SQL_BEGIN_TRANSACTION: &str = "SqlBeginTransaction";
pub const OP_SQL_IS_IN_TRANSACTION: &str = "SqlIsInTransaction";
pub const OP_SQL_CANCEL_TRANSACTION: &str = "SqlCancelTransaction";
pub const OP_SET_AUTH_OPS_BYTES: &str = "SetAuthOpsBytes";
pub const OP_QUERY_USER_LOGIN_SESSION_KEY: &str = "QueryUserLoginSessionKeyRequest";
pub const OP_GET_FILED_PAYMENTS: &str = "GetFailedPayments";
pub const OP_SET_FILED_PAYMENTS: &str = "AppendFailedPayments";
pub const OP_DUMP_GLOBAL_STATES: &str = "DumpGlobalsStates";
pub const OP_DUMP_TAPP_STATES: &str = "DumpTappStates";
pub const OP_DUMP_GLUEDB_DATA: &str = "DumpGluedbData";
pub const OP_DUMP_RAW_STATE: &str = "DumpRawState";
pub const OP_EXTEND_AUTH_KEY: &str = "ExtendAuthKey";
pub const OP_GET_MAGIC_NUMBER: &str = "GetMagicNumber";

pub const OP_BONDING_BUY: &str = "BondingBuy";
pub const OP_BONDING_SELL: &str = "BondingSell";
pub const OP_READ_BONDING_TOTAL_SUPPLY: &str = "ReadBondingTotalSupply";
pub const OP_READ_ALL_BONDING: &str = "ReadAllBonding";
pub const OP_GET_BONDING_TOTAL_SUPPLY: &str = "GetBondingTotalSupply";
pub const OP_QUERY_TOKEN_BALANCE: &str = "QueryTokenBalance";
pub const OP_READ_TOKEN_BALANCE: &str = "ReadTokenBalance";

pub const OP_GET_TOKEN_RESERVED_BALANCE: &str = "GetTokenReservedBalance";
pub const OP_BONDING_RESERVE: &str = "BondingReserve";
pub const OP_BONDING_UNRESERVE: &str = "BondingUnreserve";
pub const OP_READ_BONDING_RESERVE: &str = "ReadBondingReserve";
