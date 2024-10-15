use zstd_rust::frame::Frame;
use zstd_rust::parsing::ForwardByteParser;
#[test]
fn parse_skippable_frame() {
    let mock_data = [
        // Skippable frame with magic 0x184d2a53, length 3, content 0x10 0x20 0x30
        // and an extra byte at the end.
        0x55, 0x2a, 0x4d, 0x18, 0x03, 0x00, 0x00, 0x00, 0x10, 0x20, 0x30, 0x40,
        //^--- magic (LE) ----^ ^------ 3 (LE) -------^ ^--- content ---^ ^-- extra
    ];

    let mut parser = ForwardByteParser::new(&mock_data);

    let Frame::SkippableFrame(skippable) = Frame::parse(&mut parser).unwrap() else {
        panic!("unexpected frame type")
    };
    assert_eq!(0x184d2a55, skippable.magic);
    assert_eq!(&[0x10, 0x20, 0x30], skippable.data);
    assert_eq!(1, parser.len());
}