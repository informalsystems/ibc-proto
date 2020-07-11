/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between ICS20 enabled chains.
/// See ICS Spec here: https://github.com/cosmos/ics/tree/master/spec/ics-020-fungible-token-transfer#data-structures
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag="1")]
    pub source_port: std::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag="2")]
    pub source_channel: std::string::String,
    /// the tokens to be transferred
    #[prost(message, repeated, tag="3")]
    pub amount: ::std::vec::Vec<super::super::cosmos::Coin>,
    /// the sender address
    #[prost(bytes, tag="4")]
    pub sender: std::vec::Vec<u8>,
    /// the recipient address on the destination chain
    #[prost(string, tag="5")]
    pub receiver: std::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag="6")]
    pub timeout_height: u64,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag="7")]
    pub timeout_timestamp: u64,
}
/// FungibleTokenPacketData defines a struct for the packet payload
/// See FungibleTokenPacketData spec: https://github.com/cosmos/ics/tree/master/spec/ics-020-fungible-token-transfer#data-structures
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FungibleTokenPacketData {
    /// the tokens to be transferred
    #[prost(message, repeated, tag="1")]
    pub amount: ::std::vec::Vec<super::super::cosmos::Coin>,
    /// the sender address
    #[prost(string, tag="2")]
    pub sender: std::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag="3")]
    pub receiver: std::string::String,
}
/// FungibleTokenPacketAcknowledgement contains a boolean success flag and an optional error msg
/// error msg is empty string on success
/// See spec for onAcknowledgePacket: https://github.com/cosmos/ics/tree/master/spec/ics-020-fungible-token-transfer#packet-relay
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FungibleTokenPacketAcknowledgement {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub error: std::string::String,
}
