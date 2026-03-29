#[derive(Debug)]
pub struct BitBuffer {
    data: Vec<u8>,
    bit_len: usize,
    max_bits: u32,
}

impl BitBuffer {
    pub fn new(max_bits: u32) -> Self {
        Self { data: Vec::new(), bit_len: 0, max_bits }
    }

    pub fn write_bit(&mut self, bit: bool) -> () {
        // Find target byte
        let byte_index = self.bit_len / 8;

        // Ensure capacity
        if byte_index == self.data.len() {
            self.data.push(0);
        }

        // Find the bit position. The poisition from which we can start writing new bits
        let bit_index = 7 - (self.bit_len % 8);

        // Set the bit
        if bit {
            // Shift 1 bit_index bits over.
            let mask = 1 << bit_index;

            // Perform OR with mask to set the bits as necessary
            // Already set bits will be ignored.
            self.data[byte_index] |= mask;
        }

        // Advance the bit index
        self.bit_len += 1;
    }

    pub fn write_bits(&mut self, bits: u32, bit_count: u8) -> () {
        for i in (0..bit_count).rev() {
            let bit = ((bits >> i) & 1) != 0;
            self.write_bit(bit);
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> () {
        self.write_bits(byte as u32, 8)
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> () {
        bytes.iter().for_each(|byte| self.write_byte(byte.clone()))
    }

    pub fn pad_to_byte(&mut self) -> () {
        let padding = self.bit_len % 8;

        // Shift the bit_len to the nearest byte
        self.bit_len += padding;
    }

    pub fn write_zeros(&mut self, count: usize) -> () {
        for _ in 0..count {
            self.data.push(0);
        }
    }

    // Capacity Helpers
    pub fn bit_len(&self) -> &usize {
        &self.bit_len
    }

    pub fn byte_len(&self) -> usize {
        self.bit_len / 8
    }

    pub fn remaining_bits(&self) -> u32 {
        self.max_bits - (self.bit_len as u32)
    }
}
