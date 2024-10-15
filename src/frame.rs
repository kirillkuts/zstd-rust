use crate::parsing::ForwardByteParser;

pub enum Frame{
    ZstandardFrame,
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

        if magic_number == DATA_FRAME_MAGIC_NUMBER {
            return Ok(Frame::ZstandardFrame);
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