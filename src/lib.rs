#[derive(Debug, PartialEq, Eq)]
pub struct GoLField {
    field_data: std::vec::Vec<u8>,
    width: u32,
}

impl GoLField {
    /*1,1 = erstes byte, fünftes bit
        0, 0, 0,
        0, 1, 0,
        0, 0, 0

        00001000 00000000  

         */
    fn get_relbyte_and_bits_to_shift(&self, width: u32, height: u32) -> (usize, u32) {
        assert!(width < self.width);
        let bit = (self.width * height) + width;
        let byte_no = (bit as f32 / 8_f32).floor() as usize;
        assert!(byte_no < self.field_data.len());
        let bit_to_shift = 7 - (bit % 8);
        assert!(bit_to_shift < 8);
        (byte_no, bit_to_shift)
    }
    pub fn new(width: u32, height: u32) -> GoLField {
        let needed_bytes = (width * height) as f32 / 8_f32;
        let needed_bytes = needed_bytes.ceil();
        let field = vec![0_u8; needed_bytes as usize];

        GoLField {
            field_data: field,
            width: width,
        }
    }
    pub fn get_cell(&self, width: u32, height: u32) -> bool {
        let bit_stuff = self.get_relbyte_and_bits_to_shift(width, height);
        let relevant_byte = self.field_data[bit_stuff.0];

        relevant_byte & (0x01 << bit_stuff.1) != 0x00
    }
    pub fn set_cell_alive(&mut self, width: u32, height: u32) {
        let bit_stuff = self.get_relbyte_and_bits_to_shift(width, height);

        if let Some(relevant_byte) = self.field_data.get_mut(bit_stuff.0) {
            *relevant_byte = *relevant_byte | (0x1 << bit_stuff.1);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    #[test]
    fn get_small_gol() {
        let mut field = super::GoLField::new(10, 10);
        for h in 0..10 {
            for w in 0..10 {
                field.set_cell_alive(w, h);
            }
        }
        assert_eq!(
            field.field_data,
            &[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 240]
        )
    }
}
