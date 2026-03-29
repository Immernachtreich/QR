use std::cmp::min;

use crate::util::bit_buffer::{ BitBuffer };

#[derive(Debug)]
pub struct QRCodeEncoder {
    bit_buffer: BitBuffer,
    mode: u8,
}

impl QRCodeEncoder {
    pub fn new(data: &str, version: usize) -> Self {
        let mut qr_code = Self {
            bit_buffer: BitBuffer::new(152),
            mode: 0b0100,
        };

        qr_code
            .write_mode_indicator()
            .write_char_count(data)
            .write_data(data)
            .write_terminator()
            .write_to_byte_to_boundary()
            .write_padding_bytes();

        qr_code
    }

    fn write_mode_indicator(&mut self) -> &mut Self {
        self.bit_buffer.write_bits(self.mode as u32, 4);
        self
    }

    fn write_char_count(&mut self, data: &str) -> &mut Self {
        let length = data.chars().count();
        self.bit_buffer.write_byte(length as u8);

        self
    }

    fn write_data(&mut self, data: &str) -> &mut Self {
        data.chars().for_each(|c| {
            let ascii_code = c as u8;
            self.bit_buffer.write_byte(ascii_code);
        });

        self
    }

    fn write_terminator(&mut self) -> &mut Self {
        let terminator_bits = min(4, self.bit_buffer.remaining_bits());

        for _ in 0..terminator_bits {
            self.bit_buffer.write_bit(false);
        }

        self
    }

    fn write_to_byte_to_boundary(&mut self) -> &mut Self {
        self.bit_buffer.pad_to_byte();

        self
    }

    fn write_padding_bytes(&mut self) -> &mut Self {
        let pad_byte_a = 0b11101100;
        let pad_byte_b = 0b00010001;
        let mut alternate = false;

        while self.bit_buffer.remaining_bits() > 0 {
            if alternate {
                self.bit_buffer.write_byte(pad_byte_b);
            } else {
                self.bit_buffer.write_byte(pad_byte_a);
            }

            alternate = !alternate;
        }

        self
    }
}
