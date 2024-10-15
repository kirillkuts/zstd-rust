use zstd_rust::parsing::{ForwardByteParser};

#[test]
fn forward_byte_parser_u8() {
    // Check that bytes are delivered in order
    let mut parser = ForwardByteParser::new(&[0x12, 0x23, 0x34]);
    assert_eq!(0x12, parser.u8().unwrap());
    assert_eq!(0x23, parser.u8().unwrap());
    assert_eq!(0x34, parser.u8().unwrap());
    assert!(matches!(
        parser.u8(),
        None
    ));
}