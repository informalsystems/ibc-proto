//----------------------------------------
// Message types

/// ProofOp defines an operation used for calculating Merkle root
/// The data could be arbitrary format, providing nessecary data
/// for example neighbouring node hash
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProofOp {
    #[prost(string, tag="1")]
    pub r#type: std::string::String,
    #[prost(bytes, tag="2")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="3")]
    pub data: std::vec::Vec<u8>,
}
/// Proof is Merkle proof defined by the list of ProofOps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proof {
    #[prost(message, repeated, tag="1")]
    pub ops: ::std::vec::Vec<ProofOp>,
}
