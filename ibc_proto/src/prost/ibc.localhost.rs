/// MsgCreateClient defines a message to create a localhost client state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClient {
    #[prost(bytes, tag="1")]
    pub signer: std::vec::Vec<u8>,
}
/// ClientState defines a loopback (localhost) client. It requires (read-only)
/// access to keys outside the client prefix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// client id
    #[prost(string, tag="1")]
    pub id: std::string::String,
    /// self chain ID
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    /// self latest block height
    #[prost(uint64, tag="3")]
    pub height: u64,
}
