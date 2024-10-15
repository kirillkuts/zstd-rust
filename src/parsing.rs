pub struct ForwardByteParser<'a>(&'a [u8]);

#[derive(Debug)]
pub enum ParserError {
    NotEnoughBytes,
    // Add more error variants if necessary
}

impl<'a> ForwardByteParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }

    pub fn u8(&mut self) -> Option<u8> {
        let (first, rest) = self.0.split_first()?;
        self.0 = rest;
        Some(*first)
    }

    /// Return the number of bytes still unparsed
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Check if the input is exhausted
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Extract `len` bytes as a slice
    pub fn slice(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.len() >= len {
            let (result, rest) = self.0.split_at(len);
            self.0 = rest;
            return Some(result);
        }

        None
    }

    /// Consume and return a u32 in little-endian format
    pub fn le_u32(&mut self) -> Result<u32, ParserError> {
        if let Some(slice) = self.slice(4) {
            // Convert using big-endian (most significant byte first)
            let numer = u32::from_le_bytes(slice.try_into().expect("slice should be 4 bytes"));
            Ok(numer)
        } else {
            Err(ParserError::NotEnoughBytes)
        }
    }
}