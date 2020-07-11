/// MsgCreateClient defines a message to create an IBC client
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClient {
    #[prost(bytes, tag="1")]
    pub signer: std::vec::Vec<u8>,
}
