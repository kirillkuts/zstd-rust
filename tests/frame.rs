use zstd_rust::frame::{Frame, FrameHeader};
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
    // assert_eq!(&[0x10, 0x20, 0x30], skippable.data);
    assert_eq!(1, parser.len());
}

#[test]
fn parse_frame_header() {
    let mock_data = [
        0x87 // 10000111
             // (7-6 bits) 10 - Frame_Content_Size_Flag "2" -> FCS_Field_Size = "4"
             // (5th fit)   0 - Single_Segment_Flag
             // (4-3)       0 - unused/reserved
             // (2nd bit)   1 - content checksum will be present at the frame's end.
             // (1-0 bits) 11 - Dictionary_ID_Flag = 3 -> DID_Field_Size = 4
    ];

    let mut parser = ForwardByteParser::new(&mock_data);

    let frame_header = FrameHeader::parse(&mut parser);

    assert_eq!(frame_header.is_single_segment, false);
    assert_eq!(frame_header.has_content_checksum, true);
    assert_eq!(frame_header.fcs_field_size, 4);
    assert_eq!(frame_header.did_field_size, 4);

    let mock_data = [
        0x24 // 10000111
             // (7-6 bits) 10 - Frame_Content_Size_Flag "2" -> FCS_Field_Size = "4"
             // (5th fit)   0 - Single_Segment_Flag
             // (4-3)       0 - unused/reserved
             // (2nd bit)   1 - content checksum will be present at the frame's end.
             // (1-0 bits) 11 - Dictionary_ID_Flag = 3 -> DID_Field_Size = 4
    ];

    let mut parser = ForwardByteParser::new(&mock_data);

    let frame_header = FrameHeader::parse(&mut parser);

    assert_eq!(frame_header.is_single_segment, true);
    assert_eq!(frame_header.has_content_checksum, true);
    assert_eq!(frame_header.fcs_field_size, 1);
    assert_eq!(frame_header.did_field_size, 0);
}