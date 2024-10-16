
use crate::parsing::ForwardByteParser;

pub enum Block {
    Raw { size: usize, data: Vec<u8> },
    RLE { byte: u8, repeat: usize },
    Compressed,
}
impl Block {
    pub fn parse(input: &mut ForwardByteParser) -> (bool, Block) {
        let bytes_slice = input.slice(3).unwrap();

        let mut bytes = [0u8; 4];  // Create a 4-byte array initialized with zeros
        bytes[..3].copy_from_slice(bytes_slice);

        let data = u32::from_le_bytes(bytes.try_into().unwrap());

        let is_last = data & 1 == 1;
        let block_type = (data >> 1) & 3;
        let block_size = (data >> 3) as usize;

        let block = match block_type {
            0 => {
                let data = input.slice(block_size).unwrap();
                Block::Raw { size: block_size, data: data.to_owned() }
            },
            1 => {
                let byte = input.u8().unwrap();
                Block::RLE { repeat: block_size, byte }
            },
            // 1 => Block::Raw { size: block_size },
            _ => panic!("Unknown block_type."),
        };

        (is_last, block)
    }

    pub fn decode(&self) -> Option<Vec<u8>> {
        match self {
            Block::Raw { size, data } => {
                Some(data.as_slice().to_owned())
            },
            Block::RLE { byte, repeat } => {
                Some(std::iter::repeat(*byte).take(repeat.to_owned()).collect::<Vec<u8>>())
            },
            _ => panic!("Unknown block_type."),
        }
    }
}