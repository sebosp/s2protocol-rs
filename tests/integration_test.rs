use nom_mpq::parser;

#[test_log::test]
fn s2_protocol_87702_version() {
    let file_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/SC2-Patch_4.12-2v2AI.SC2Replay"
    );
    let file_contents = parser::read_file(file_path);
    let (_input, mpq) = parser::parse(&file_contents).unwrap();
    let (_tail, proto_header) = s2protocol::read_protocol_header(&mpq).unwrap();
    assert_eq!(proto_header.m_signature, b"StarCraft II replay\x1b11"[..]);
    assert_eq!(proto_header.m_version.m_base_build, 87702);
}
