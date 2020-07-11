//----------------------------------------
// Request types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Value", tags="2, 3, 4, 5, 6, 7, 8, 9, 19, 11, 12")]
    pub value: ::std::option::Option<request::Value>,
}
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="2")]
        Echo(super::RequestEcho),
        #[prost(message, tag="3")]
        Flush(super::RequestFlush),
        #[prost(message, tag="4")]
        Info(super::RequestInfo),
        #[prost(message, tag="5")]
        SetOption(super::RequestSetOption),
        #[prost(message, tag="6")]
        InitChain(super::RequestInitChain),
        #[prost(message, tag="7")]
        Query(super::RequestQuery),
        #[prost(message, tag="8")]
        BeginBlock(super::RequestBeginBlock),
        #[prost(message, tag="9")]
        CheckTx(super::RequestCheckTx),
        #[prost(message, tag="19")]
        DeliverTx(super::RequestDeliverTx),
        #[prost(message, tag="11")]
        EndBlock(super::RequestEndBlock),
        #[prost(message, tag="12")]
        Commit(super::RequestCommit),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEcho {
    #[prost(string, tag="1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFlush {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInfo {
    #[prost(string, tag="1")]
    pub version: std::string::String,
    #[prost(uint64, tag="2")]
    pub block_version: u64,
    #[prost(uint64, tag="3")]
    pub p2p_version: u64,
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSetOption {
    #[prost(string, tag="1")]
    pub key: std::string::String,
    #[prost(string, tag="2")]
    pub value: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInitChain {
    #[prost(message, optional, tag="1")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub consensus_params: ::std::option::Option<ConsensusParams>,
    #[prost(message, repeated, tag="4")]
    pub validators: ::std::vec::Vec<ValidatorUpdate>,
    #[prost(bytes, tag="5")]
    pub app_state_bytes: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuery {
    #[prost(bytes, tag="1")]
    pub data: std::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub path: std::string::String,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(bool, tag="4")]
    pub prove: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBeginBlock {
    #[prost(bytes, tag="1")]
    pub hash: std::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub header: ::std::option::Option<Header>,
    #[prost(message, optional, tag="3")]
    pub last_commit_info: ::std::option::Option<LastCommitInfo>,
    #[prost(message, repeated, tag="4")]
    pub byzantine_validators: ::std::vec::Vec<Evidence>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCheckTx {
    #[prost(bytes, tag="1")]
    pub tx: std::vec::Vec<u8>,
    #[prost(enumeration="CheckTxType", tag="2")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDeliverTx {
    #[prost(bytes, tag="1")]
    pub tx: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEndBlock {
    #[prost(int64, tag="1")]
    pub height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCommit {
}
//----------------------------------------
// Response types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub value: ::std::option::Option<response::Value>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        Exception(super::ResponseException),
        #[prost(message, tag="2")]
        Echo(super::ResponseEcho),
        #[prost(message, tag="3")]
        Flush(super::ResponseFlush),
        #[prost(message, tag="4")]
        Info(super::ResponseInfo),
        #[prost(message, tag="5")]
        SetOption(super::ResponseSetOption),
        #[prost(message, tag="6")]
        InitChain(super::ResponseInitChain),
        #[prost(message, tag="7")]
        Query(super::ResponseQuery),
        #[prost(message, tag="8")]
        BeginBlock(super::ResponseBeginBlock),
        #[prost(message, tag="9")]
        CheckTx(super::ResponseCheckTx),
        #[prost(message, tag="10")]
        DeliverTx(super::ResponseDeliverTx),
        #[prost(message, tag="11")]
        EndBlock(super::ResponseEndBlock),
        #[prost(message, tag="12")]
        Commit(super::ResponseCommit),
    }
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseException {
    #[prost(string, tag="1")]
    pub error: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEcho {
    #[prost(string, tag="1")]
    pub message: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlush {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInfo {
    #[prost(string, tag="1")]
    pub data: std::string::String,
    #[prost(string, tag="2")]
    pub version: std::string::String,
    #[prost(uint64, tag="3")]
    pub app_version: u64,
    #[prost(int64, tag="4")]
    pub last_block_height: i64,
    #[prost(bytes, tag="5")]
    pub last_block_app_hash: std::vec::Vec<u8>,
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseSetOption {
    #[prost(uint32, tag="1")]
    pub code: u32,
    /// bytes data = 2;
    #[prost(string, tag="3")]
    pub log: std::string::String,
    #[prost(string, tag="4")]
    pub info: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInitChain {
    #[prost(message, optional, tag="1")]
    pub consensus_params: ::std::option::Option<ConsensusParams>,
    #[prost(message, repeated, tag="2")]
    pub validators: ::std::vec::Vec<ValidatorUpdate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuery {
    #[prost(uint32, tag="1")]
    pub code: u32,
    /// bytes data = 2; // use "value" instead.
    ///
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: std::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: std::string::String,
    #[prost(int64, tag="5")]
    pub index: i64,
    #[prost(bytes, tag="6")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="7")]
    pub value: std::vec::Vec<u8>,
    #[prost(message, optional, tag="8")]
    pub proof: ::std::option::Option<super::super::crypto::merkle::Proof>,
    #[prost(int64, tag="9")]
    pub height: i64,
    #[prost(string, tag="10")]
    pub codespace: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBeginBlock {
    #[prost(message, repeated, tag="1")]
    pub events: ::std::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCheckTx {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: std::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: std::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="7")]
    pub events: ::std::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDeliverTx {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: std::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: std::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="7")]
    pub events: ::std::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEndBlock {
    #[prost(message, repeated, tag="1")]
    pub validator_updates: ::std::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag="2")]
    pub consensus_param_updates: ::std::option::Option<ConsensusParams>,
    #[prost(message, repeated, tag="3")]
    pub events: ::std::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCommit {
    /// reserve 1
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
}
//----------------------------------------
// Misc.

/// ConsensusParams contains all consensus-relevant parameters
/// that can be adjusted by the abci app
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParams {
    #[prost(message, optional, tag="1")]
    pub block: ::std::option::Option<BlockParams>,
    #[prost(message, optional, tag="2")]
    pub evidence: ::std::option::Option<EvidenceParams>,
    #[prost(message, optional, tag="3")]
    pub validator: ::std::option::Option<ValidatorParams>,
}
/// BlockParams contains limits on the block size.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParams {
    /// Note: must be greater than 0
    #[prost(int64, tag="1")]
    pub max_bytes: i64,
    /// Note: must be greater or equal to -1
    #[prost(int64, tag="2")]
    pub max_gas: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvidenceParams {
    /// Note: must be greater than 0
    #[prost(int64, tag="1")]
    pub max_age_num_blocks: i64,
    #[prost(message, optional, tag="2")]
    pub max_age_duration: ::std::option::Option<::prost_types::Duration>,
}
/// ValidatorParams contains limits on validators.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorParams {
    #[prost(string, repeated, tag="1")]
    pub pub_key_types: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastCommitInfo {
    #[prost(int32, tag="1")]
    pub round: i32,
    #[prost(message, repeated, tag="2")]
    pub votes: ::std::vec::Vec<VoteInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub r#type: std::string::String,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::std::vec::Vec<super::super::libs::kv::Pair>,
}
//----------------------------------------
// Blockchain Types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// basic block info
    #[prost(message, optional, tag="1")]
    pub version: ::std::option::Option<Version>,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(message, optional, tag="4")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag="5")]
    pub last_block_id: ::std::option::Option<BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes, tag="6")]
    pub last_commit_hash: std::vec::Vec<u8>,
    /// transactions
    #[prost(bytes, tag="7")]
    pub data_hash: std::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes, tag="8")]
    pub validators_hash: std::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes, tag="9")]
    pub next_validators_hash: std::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes, tag="10")]
    pub consensus_hash: std::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes, tag="11")]
    pub app_hash: std::vec::Vec<u8>,
    /// root hash of all results from the txs from the previous block
    #[prost(bytes, tag="12")]
    pub last_results_hash: std::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes, tag="13")]
    pub evidence_hash: std::vec::Vec<u8>,
    /// original proposer of the block
    #[prost(bytes, tag="14")]
    pub proposer_address: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(uint64, tag="1")]
    pub block: u64,
    #[prost(uint64, tag="2")]
    pub app: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockId {
    #[prost(bytes, tag="1")]
    pub hash: std::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub parts_header: ::std::option::Option<PartSetHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartSetHeader {
    #[prost(int32, tag="1")]
    pub total: i32,
    #[prost(bytes, tag="2")]
    pub hash: std::vec::Vec<u8>,
}
/// Validator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(bytes, tag="1")]
    pub address: std::vec::Vec<u8>,
    /// PubKey pub_key = 2 [(gogoproto.nullable)=false];
    #[prost(int64, tag="3")]
    pub power: i64,
}
/// ValidatorUpdate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorUpdate {
    #[prost(message, optional, tag="1")]
    pub pub_key: ::std::option::Option<PubKey>,
    #[prost(int64, tag="2")]
    pub power: i64,
}
/// VoteInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteInfo {
    #[prost(message, optional, tag="1")]
    pub validator: ::std::option::Option<Validator>,
    #[prost(bool, tag="2")]
    pub signed_last_block: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(string, tag="1")]
    pub r#type: std::string::String,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evidence {
    #[prost(string, tag="1")]
    pub r#type: std::string::String,
    #[prost(message, optional, tag="2")]
    pub validator: ::std::option::Option<Validator>,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(message, optional, tag="4")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag="5")]
    pub total_voting_power: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckTxType {
    New = 0,
    Recheck = 1,
}
