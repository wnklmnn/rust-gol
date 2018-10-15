#[derive(Debug, PartialEq, Eq)]
pub struct GoLField {
    field_data: std::vec::Vec<u8>,
    width: u32,
    height: u32,
}

pub enum EdgeBehavior {
    Wrapping,
    DeadCells,
}

impl GoLField {
    /*1,1 = erstes byte, fünftes bit
        0, 0, 0,
        0, 1, 0,
        0, 0, 0

        0000_1000 0000_0000  

         */

    fn get_alive_neightbour_count(&self, width: u32, height: u32, eb: &EdgeBehavior) -> u8 {
        let width: i64 = i64::from(width);
        let height: i64 = i64::from(height);
        let mut count: u8 = 0;
        match eb {
            EdgeBehavior::Wrapping => {
                for h in -1..2 {
                    for w in -1..2 {
                        if w == 0 && h == 0 {
                            continue;
                        }

                        let target_width = width + w; // Diese Zeile muss noch angepassst werden
                        let target_height = height + h; // Diese Zeile muss noch angepassst werden

                        if self.get_cell(target_width as u32, target_height as u32) == true {
                            count += 1;
                        }
                    }
                }

                println!("Oh nooo");
                unimplemented!()
            }
            EdgeBehavior::DeadCells => {
                /* 
                1,1,0
                1,1,0
                0,0,0
                */
                for h in -1..2 {
                    for w in -1..2 {
                        if w == 0 && h == 0 {
                            continue;
                        }
                        let target_width = width + w;
                        let target_height = height + h;
                        if target_width < 0 || target_width >= i64::from(self.width) {
                            continue;
                        }
                        if target_height < 0 || target_height >= i64::from(self.height) {
                            continue;
                        }
                        if self.get_cell(target_width as u32, target_height as u32) == true {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
    pub fn calc_next_iteration(&self, eb: EdgeBehavior) -> GoLField {
        let mut ret = GoLField::new(self.width, self.height);
        for h in 0..self.height {
            for w in 0..self.width {
                let al_neigthbours = self.get_alive_neightbour_count(w, h, &eb);
                println!("al_neightbours({}, {}) = {}", w, h, al_neigthbours);
                match al_neigthbours {
                    0...1 => {
                        //Zelle wird nicht belebt
                    }
                    2 => {
                        if self.get_cell(w, h) {
                            ret.set_cell_alive(w, h);
                        }
                    }
                    3 => {
                        ret.set_cell_alive(w, h);
                    }
                    4...8 => {
                        //Zelle wird nicht belebt
                    }
                    _ => unreachable!(),
                }
            }
        }
        ret
    }

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
            width,
            height,
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
            *relevant_byte |= 0x1 << bit_stuff.1;
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
    #[test]
    fn iter_simple() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(1, 1);
        assert_eq!(field.field_data, &[8_u8, 0_u8]);
        let new_field = field.calc_next_iteration(super::EdgeBehavior::DeadCells);
        assert_eq!(new_field.field_data, &[0_u8, 0_u8]);
    }

    #[test]
    fn iter_simple_static_block() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(0, 0);
        field.set_cell_alive(0, 1);
        field.set_cell_alive(1, 1);
        field.set_cell_alive(1, 0);
        assert_eq!(field.field_data, &[216_u8, 0_u8]);
        let new_field = field.calc_next_iteration(super::EdgeBehavior::DeadCells);
        assert_eq!(new_field.field_data, &[216_u8, 0_u8]);
    }

    #[test]
    fn iter_blink() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(1, 0); //010
        field.set_cell_alive(1, 1); //010
        field.set_cell_alive(1, 2); //010
        assert_eq!(field.field_data, &[73_u8, 0_u8]); //0100_1001_0000_0000
        let new_field = field.calc_next_iteration(super::EdgeBehavior::DeadCells);
        /*
        000
        111
        000

        0001_1100_0000_0000
         */
        assert_eq!(new_field.field_data, &[28_u8, 0_u8]);
    }

}
