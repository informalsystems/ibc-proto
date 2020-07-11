/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag="1")]
    pub denom: std::string::String,
    #[prost(string, tag="2")]
    pub amount: std::string::String,
}
/// DecCoin defines a token with a denomination and a decimal amount.
///
/// NOTE: The amount field is an Dec which implements the custom method
/// signatures required by gogoproto.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecCoin {
    #[prost(string, tag="1")]
    pub denom: std::string::String,
    #[prost(string, tag="2")]
    pub amount: std::string::String,
}
/// IntProto defines a Protobuf wrapper around an Int object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntProto {
    #[prost(string, tag="1")]
    pub int: std::string::String,
}
/// DecProto defines a Protobuf wrapper around a Dec object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecProto {
    #[prost(string, tag="1")]
    pub dec: std::string::String,
}
/// ValAddresses defines a repeated set of validator addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValAddresses {
    #[prost(bytes, repeated, tag="1")]
    pub addresses: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// GasInfo defines tx execution gas context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    #[prost(uint64, tag="1")]
    pub gas_wanted: u64,
    /// GasUsed is the amount of gas actually consumed.
    #[prost(uint64, tag="2")]
    pub gas_used: u64,
}
/// Result is the union of ResponseFormat and ResponseCheckTx.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    /// Data is any data returned from message or handler execution. It MUST be length
    /// prefixed in order to separate data from multiple message executions.
    #[prost(bytes, tag="1")]
    pub data: std::vec::Vec<u8>,
    /// Log contains the log information from message or handler execution.
    #[prost(string, tag="2")]
    pub log: std::string::String,
    /// Events contains a slice of Event objects that were emitted during message or
    /// handler execution.
    #[prost(message, repeated, tag="3")]
    pub events: ::std::vec::Vec<super::tendermint::abci::types::Event>,
}
/// SimulationResponse defines the response generated when a transaction is
/// successfully simulated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulationResponse {
    #[prost(message, optional, tag="1")]
    pub gas_info: ::std::option::Option<GasInfo>,
    #[prost(message, optional, tag="2")]
    pub result: ::std::option::Option<Result>,
}
/// MsgData defines the data returned in a Result object during message execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgData {
    #[prost(string, tag="1")]
    pub msg_type: std::string::String,
    #[prost(bytes, tag="2")]
    pub data: std::vec::Vec<u8>,
}
/// TxData defines a list of MsgData. A transaction will have a MsgData object for
/// each message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxData {
    #[prost(message, repeated, tag="1")]
    pub data: ::std::vec::Vec<MsgData>,
}
