/// MsgConnectionOpenInit defines the msg sent by an account on Chain A to
/// initialize a connection with Chain B.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenInit {
    #[prost(string, tag="1")]
    pub client_id: std::string::String,
    #[prost(string, tag="2")]
    pub connection_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub counterparty: ::std::option::Option<Counterparty>,
    #[prost(bytes, tag="4")]
    pub signer: std::vec::Vec<u8>,
}
/// MsgConnectionOpenTry defines a msg sent by a Relayer to try to open a connection
/// on Chain B.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenTry {
    #[prost(string, tag="1")]
    pub client_id: std::string::String,
    #[prost(string, tag="2")]
    pub connection_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub counterparty: ::std::option::Option<Counterparty>,
    #[prost(string, repeated, tag="4")]
    pub counterparty_versions: ::std::vec::Vec<std::string::String>,
    /// proof of the initialization the connection on Chain A: `UNITIALIZED -> INIT`
    #[prost(bytes, tag="5")]
    pub proof_init: std::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub proof_height: u64,
    /// proof of client consensus state
    #[prost(bytes, tag="7")]
    pub proof_consensus: std::vec::Vec<u8>,
    #[prost(uint64, tag="8")]
    pub consensus_height: u64,
    #[prost(bytes, tag="9")]
    pub signer: std::vec::Vec<u8>,
}
/// MsgConnectionOpenAck defines a msg sent by a Relayer to Chain A to acknowledge
/// the change of connection state to TRYOPEN on Chain B.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenAck {
    #[prost(string, tag="1")]
    pub connection_id: std::string::String,
    #[prost(string, tag="2")]
    pub version: std::string::String,
    /// proof of the initialization the connection on Chain B: `UNITIALIZED -> TRYOPEN`
    #[prost(bytes, tag="3")]
    pub proof_try: std::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub proof_height: u64,
    /// proof of client consensus state
    #[prost(bytes, tag="5")]
    pub proof_consensus: std::vec::Vec<u8>,
    #[prost(uint64, tag="6")]
    pub consensus_height: u64,
    #[prost(bytes, tag="7")]
    pub signer: std::vec::Vec<u8>,
}
/// MsgConnectionOpenConfirm defines a msg sent by a Relayer to Chain B to acknowledge
/// the change of connection state to OPEN on Chain A.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConnectionOpenConfirm {
    #[prost(string, tag="1")]
    pub connection_id: std::string::String,
    /// proof for the change of the connection state on Chain A: `INIT -> OPEN`
    #[prost(bytes, tag="2")]
    pub proof_ack: std::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub proof_height: u64,
    #[prost(bytes, tag="4")]
    pub signer: std::vec::Vec<u8>,
}
// ICS03 - Connection Data Structures as defined in
// https://github.com/cosmos/ics/tree/master/spec/ics-003-connection-semantics#data-structures

/// ConnectionEnd defines a stateful object on a chain connected to another separate
/// one.
/// NOTE: there must only be 2 defined ConnectionEnds to establish a connection
/// between two chains.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionEnd {
    /// connection identifier.
    #[prost(string, tag="1")]
    pub id: std::string::String,
    /// client associated with this connection.
    #[prost(string, tag="2")]
    pub client_id: std::string::String,
    /// opaque string which can be utilised to determine encodings or protocols for
    /// channels or packets utilising this connection
    #[prost(string, repeated, tag="3")]
    pub versions: ::std::vec::Vec<std::string::String>,
    /// current state of the connection end.
    #[prost(enumeration="State", tag="4")]
    pub state: i32,
    /// counterparty chain associated with this connection.
    #[prost(message, optional, tag="5")]
    pub counterparty: ::std::option::Option<Counterparty>,
}
/// Counterparty defines the counterparty chain associated with a connection end.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counterparty {
    /// identifies the client on the counterparty chain associated with a given connection.
    #[prost(string, tag="1")]
    pub client_id: std::string::String,
    /// identifies the connection end on the counterparty chain associated with a given connection.
    #[prost(string, tag="2")]
    pub connection_id: std::string::String,
    /// commitment merkle prefix of the counterparty chain
    #[prost(message, optional, tag="3")]
    pub prefix: ::std::option::Option<super::commitment::MerklePrefix>,
}
/// ClientPaths define all the connection paths for a client state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientPaths {
    /// list of connection paths
    #[prost(string, repeated, tag="1")]
    pub paths: ::std::vec::Vec<std::string::String>,
}
/// ConnectionPaths define all the connection paths for a given client state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionPaths {
    /// client state unique identifier
    #[prost(string, tag="1")]
    pub client_id: std::string::String,
    /// list of connection paths
    #[prost(string, repeated, tag="2")]
    pub paths: ::std::vec::Vec<std::string::String>,
}
/// State defines if a connection is in one of the following states:
/// INIT, TRYOPEN, OPEN or UNINITIALIZED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A connection end has just started the opening handshake.
    Init = 1,
    /// A connection end has acknowledged the handshake step on the counterparty chain.
    Tryopen = 2,
    /// A connection end has completed the handshake.
    Open = 3,
}
/// QueryConnectionRequest is the request type for the Query/Connection RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionRequest {
    /// connection unique identifier
    #[prost(string, tag="1")]
    pub connection_id: std::string::String,
}
/// QueryConnectionResponse is the response type for the Query/Connection RPC method.
/// Besides the connection end, it includes a proof and the height from which the
/// proof was retrieved.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionResponse {
    /// connection associated with the request identifier
    #[prost(message, optional, tag="1")]
    pub connection: ::std::option::Option<ConnectionEnd>,
    /// merkle proof of existence
    #[prost(bytes, tag="2")]
    pub proof: std::vec::Vec<u8>,
    /// merkle proof path
    #[prost(string, tag="3")]
    pub proof_path: std::string::String,
    /// height at which the proof was retrieved
    #[prost(uint64, tag="4")]
    pub proof_height: u64,
}
/// QueryConnectionsRequest is the request type for the Query/Connections RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionsRequest {
    #[prost(message, optional, tag="1")]
    pub req: ::std::option::Option<super::super::cosmos::query::PageRequest>,
}
/// QueryConnectionsResponse is the response type for the Query/Connections RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionsResponse {
    /// list of stored connections of the chain.
    #[prost(message, repeated, tag="1")]
    pub connections: ::std::vec::Vec<ConnectionEnd>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub res: ::std::option::Option<super::super::cosmos::query::PageResponse>,
    /// query block height
    #[prost(int64, tag="3")]
    pub height: i64,
}
/// QueryClientConnectionsRequest is the request type for the Query/ClientConnections
/// RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientConnectionsRequest {
    /// client identifier associated with a connection
    #[prost(string, tag="1")]
    pub client_id: std::string::String,
}
/// QueryClientConnectionsResponse is the response type for the Query/ClientConnections
/// RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientConnectionsResponse {
    /// slice of all the connection paths associated with a client.
    #[prost(string, repeated, tag="1")]
    pub connection_paths: ::std::vec::Vec<std::string::String>,
    /// merkle proof of existence
    #[prost(bytes, tag="2")]
    pub proof: std::vec::Vec<u8>,
    /// merkle proof path
    #[prost(string, tag="3")]
    pub proof_path: std::string::String,
    /// height at which the proof was generated
    #[prost(uint64, tag="4")]
    pub proof_height: u64,
}
