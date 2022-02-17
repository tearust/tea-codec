/// This is used when replica provider call an app contract handler actor.
/// The app contract handler actor will be called by the long running thread
/// of replciation when a txn is need to be executed
pub const OP_ACTOR_EXEC_TXN: &str = "ActorExecuteTxn";
/// Fired when replica members has removed by conveyor timeout mechanism.
pub const OP_ACTOR_REPLICAS_REMOVED: &str = "ActorReplicasRemoved";
/// Generate a sync message and then send to other replicas
/// This is called everytime when a new txn-followup pair is received
/// by this replica. And, this txn is new to this replica. It will
/// need to notify other replicas that I received a new txn.
/// Although this txn may not be *new* to other replica.
pub const OP_GEN_SYNC_MSG: &str = "GenerateSyncMessage";
/// Receive sync message from other replica.
pub const OP_REV_SYNC_MSG: &str = "ReceiveSyncMessage";
/// Call this op to clean up dead or out-of-sync other replicas
/// Those out-of-sync replicas will be remove after this call
pub const OP_CLEAR_UP: &str = "CleanUpSyncReplica";
/// How many replicas current connect to this replica
pub const OP_GET_REPLICA_COUNT: &str = "GetReplicaCount";
/// If return a empty vec, the replica is still waiting for the second
/// part of the txn - followup pair.
/// If this is the second part, the pair is completed, it will return
/// the tsid as bytes. In this case, please generate the sync message
/// to other replica, because this replica found a new txn that need
/// to notify others
pub const OP_REV_FOLLOWUP: &str = "ReceiveFollowup";
/// Similar to OP_REV_FOLLOWUP, check return value to determin whether or
/// not to continue call OP_GEN_SYNC_MSG
pub const OP_REV_TXN: &str = "ReceiveTxn";
pub const OP_FETCH_HISTORY: &str = "FetchHistory";
pub const OP_RECOVER_HISTORY: &str = "RecoverHistory";
/// Different from `OP_FETCH_HISTORY` this will not clean outdated history txns
pub const OP_GET_HISTORY_SINCE: &str = "GetHistorySince";
pub const OP_GET_LAST_HISTORY: &str = "GetLastHistory";
pub const OP_RESET_REPLICAS_COUNT: &str = "ResetReplicasCount";
pub const OP_FIND_EXECUTED_TXN: &str = "FindExecutedTxn";
pub const OP_GET_CONSENSUS_REPLICAS: &str = "GetConsensusReplicas";
pub const OP_SET_SINGLE_MODE: &str = "SetSingleMode";
pub const OP_REPORT_TXN_EXEC_ERR: &str = "ReportTxnExecError";
pub const OP_GET_EXEC_CURSOR: &str = "GetExecCursor";
pub const OP_DUMP_TXN_SEQUENCE: &str = "DumpTxnSeq";
