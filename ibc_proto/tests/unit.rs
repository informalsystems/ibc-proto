#[test]
pub fn import_localhost() {
    use ibc_proto::localhost::MsgCreateClient;
    let x = MsgCreateClient { signer: vec![] };
    assert_eq!(x.signer, vec![]);
}
