use crate::block::Block;
use crate::parsing::ForwardByteParser;

pub enum Frame{
    ZstandardFrame(ZstandardFrame),
    SkippableFrame(SkippableFrame)
}

#[derive(Debug)]
pub struct SkippableFrame {
    pub magic: u32,
    pub data: Vec<u8>
}

impl SkippableFrame {
    pub fn decode(self) -> Vec<u8> {
        self.data
    }
}

#[derive(Debug)]
pub struct ZstandardFrame {
    header: FrameHeader,
    blocks: Vec<Block>,
    checksum: Option<u32>,
}

impl ZstandardFrame {
    pub fn parse(input: &mut ForwardByteParser) -> ZstandardFrame {
        let header = FrameHeader::parse(input);

        println!("{:?}", header);

        let mut blocks: Vec<Block> = vec![];

        while let (is_last, block) = Block::parse(input) {
            blocks.push(block);

            if  is_last {
                break;
            }
        }

        let checksum = if header.has_content_checksum {
            input.le_u32().ok()
        } else {
            None
        };

        ZstandardFrame {
            header,
            blocks,
            checksum
        }
    }
}

pub struct FrameIterator<'a> {
    parser: ForwardByteParser<'a>
}

impl<'a> FrameIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { parser: ForwardByteParser::new(data) }
    }
}

impl <'a> Iterator for FrameIterator<'a>  {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        Frame::parse(&mut self.parser).ok()
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFrame,
    // Add more error variants if necessary
}

const DATA_FRAME_MAGIC_NUMBER: u32 = 0xFD2FB528;
const SKIPPABLE_FRAME_MASK: u32 = 0x184d2a50; // can be 0x184d2a5? (any last number)

impl Frame {
    pub fn parse(input: &mut ForwardByteParser) -> Result<Frame, ParseError> {
        let magic_number = input.le_u32().map_err(|_| ParseError::InvalidFrame)?;

        println!("{}", magic_number);

        if magic_number == DATA_FRAME_MAGIC_NUMBER {
            let frame = ZstandardFrame::parse(input);
            return Ok(Frame::ZstandardFrame(frame));
        }

        if magic_number & SKIPPABLE_FRAME_MASK == SKIPPABLE_FRAME_MASK {
            let length = input.le_u32().map_err(|_| ParseError::InvalidFrame)?;
            let data = input.slice(length as usize).ok_or(ParseError::InvalidFrame)?;

            return Ok(Frame::SkippableFrame(SkippableFrame {
                magic: magic_number,
                data: data.to_owned(), // makes a copy :(
            }))
        }

        Err(ParseError::InvalidFrame)
    }
}


#[derive(Debug)]
pub struct FrameHeader {
    pub has_content_checksum: bool,
    pub is_single_segment: bool,
    pub fcs_field_size: u8, // frame content field size
    pub did_field_size: u8, // dictionary field size

    pub fc_size: Option<u64>,
    pub window_descriptor: Option<u8>,
    pub dictionary_id: Option<u64>,
}

const FCS_SIZES: [u8; 4] = [0, 2, 4, 8];
const DID_SIZES: [u8; 4] = [0, 1, 2, 4];

impl FrameHeader {

    fn read_bytes(input: &mut ForwardByteParser, size: usize) -> Option<u64> {
        if size != 0 {
            let did_field_bytes = input.slice(size as usize).unwrap();
            let mut buf = [0u8; 8];
            buf[..did_field_bytes.len()].copy_from_slice(did_field_bytes);
            Some(u64::from_le_bytes(buf))
        }  else {
            None
        }
    }

    pub fn parse(input: &mut ForwardByteParser) -> FrameHeader {
        let descriptor = input.u8().expect("Could not read FrameHeader byte");

        let is_single_segment = (descriptor >> 5) & 1 == 1;

        let fcs_field_size = if is_single_segment {
            1
        } else {
            FCS_SIZES
                .get((descriptor >> 6) as usize)
                .copied()
                .unwrap_or_else(|| panic!("Unexpected fcs_flag value."))
        };

        let has_content_checksum = descriptor & 4 == 4;

        let did_field_size = DID_SIZES.get((descriptor & 3) as usize).copied().unwrap_or_else(|| panic!("Unexpected Dictionary_ID_Flag value"));



        //

        let window_descriptor = if !is_single_segment {
            let window_byte = input.u8().expect("failed to load window_descriptor byte");

            let exponent = window_byte >> 3; // last 5 bytes
            let mantissa = window_byte & 7; // first 3 bytes

            let window_base = (1 << (10 + exponent));

            Some(window_base + (window_base / 8) * mantissa)
        } else {
            None
        };

        let dictionary_id = Self::read_bytes(input, did_field_size as usize);
        let fc_size = Self::read_bytes(input, fcs_field_size as usize);

        FrameHeader {
            is_single_segment,
            has_content_checksum,
            fcs_field_size,
            did_field_size,

            fc_size,
            dictionary_id,
            window_descriptor,
        }
    }
}