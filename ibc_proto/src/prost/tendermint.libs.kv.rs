//----------------------------------------
// Abstract types

/// Define these here for compatibility but use tmlibs/kv.Pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(bytes, tag="2")]
    pub value: std::vec::Vec<u8>,
}
/// Define these here for compatibility but use tmlibs/kv.KI64Pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ki64Pair {
    #[prost(bytes, tag="1")]
    pub key: std::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub value: i64,
}
