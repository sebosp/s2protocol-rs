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

/// A replay whose header was edited by hand (`DeadGame2_recovered_*.SC2Replay` of SpawningTool,
/// 2026-11-11) is an error, not a panic that stops the scan of a whole directory.
#[test_log::test]
fn another_signature_is_not_a_replay() {
    let file_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/SC2-Patch_4.12-2v2AI.SC2Replay"
    );
    let mut file_contents = parser::read_file(file_path);
    let at = file_contents
        .windows(s2protocol::REPLAY_SIGNATURE.len())
        .position(|w| w == s2protocol::REPLAY_SIGNATURE)
        .expect("the signature is in the user data");
    file_contents[at + 18] = b'z'; // "StarCraft II replaz"
    let (_input, mpq) = parser::parse(&file_contents).unwrap();
    let err = s2protocol::read_protocol_header(&mpq).unwrap_err();
    assert!(
        matches!(err, s2protocol::S2ProtocolError::FileHeader(_)),
        "{err}"
    );
    let err = s2protocol::read_details(file_path, &mpq, &file_contents).unwrap_err();
    assert!(
        matches!(err, s2protocol::S2ProtocolError::FileHeader(_)),
        "{err}"
    );
}
