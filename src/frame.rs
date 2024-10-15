use crate::parsing::ForwardByteParser;

pub enum Frame<'b>{
    ZstandardFrame,
    SkippableFrame(SkippableFrame<'b>)
}

pub struct SkippableFrame<'c> {
    pub magic: u32,
    pub data: &'c[u8]
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFrame,
    // Add more error variants if necessary
}

const DATA_FRAME_MAGIC_NUMBER: u32 = 0xFD2FB528;
const SKIPPABLE_FRAME_MASK: u32 = 0x184d2a50; // can be 0x184d2a5? (any last number)

impl <'b>Frame<'b> {
    pub fn parse(input: &'b mut ForwardByteParser) -> Result<Self, ParseError> {
        let magic_number = input.le_u32().map_err(|_| ParseError::InvalidFrame)?;

        if magic_number == DATA_FRAME_MAGIC_NUMBER {
            return Ok(Frame::ZstandardFrame);
        }

        if magic_number & SKIPPABLE_FRAME_MASK == SKIPPABLE_FRAME_MASK {
            let length = input.le_u32().map_err(|_| ParseError::InvalidFrame)?;
            let data = input.slice(length as usize).ok_or(ParseError::InvalidFrame)?;

            return Ok(Frame::SkippableFrame(SkippableFrame {
                magic: magic_number,
                data,
            }))
        }

        Err(ParseError::InvalidFrame)
    }
}