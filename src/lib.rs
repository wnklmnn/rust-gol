#![deny(missing_docs)]
#![warn(clippy)]
//! This crate will provide fundemantal functions to implement a basic game of life
#[derive(Debug, PartialEq, Eq)]
/// This stucture will hold a basic gamestate
pub struct GoLField {
    field_data: std::vec::Vec<u8>,
    width: u32,
    height: u32,
}

#[derive(PartialEq, Eq)]
/// Represents the state of a cell
pub enum CellState {
    /// The cell is alive
    Alive,
    /// The cell is dead
    Dead,
}

/// This Enum describes how the edge of the field will be interpreted
pub enum EdgeBehavior {
    /// Using this behavior calc_next_iter will reach around the board to read the neightbour cells
    Wrapping,
    /// Using this behavior the cells outside the board will be treated as dead cells
    DeadCells,
}

impl GoLField {
    /*1,1 = erstes byte, fünftes bit
        0, 0, 0,
        0, 1, 0,
        0, 0, 0

        0000_1000 0000_0000  

         */
    /// Returns the height of the board
    pub fn get_height(&self) -> u32 {
        self.height
    }
    /// Returns the width of the board
    pub fn get_width(&self) -> u32 {
        self.width
    }

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
                        /*
                        010
                        010
                        010
                         */
                        let mut target_width = width + w;
                        let mut target_height = height + h;
                        //eprintln!("vor: width: {}+{}={}\nheight: {}+{}={}", width, w, target_width, height, h, target_height);

                        if target_height < 0 {
                            target_height = i64::from(self.height) - 1;
                        }
                        if target_height as u32 >= self.height {
                            target_height = 0;
                        }

                        if target_width < 0 {
                            //eprint!("slef.width: {}", self.width);
                            target_width = (i64::from(self.width)) - 1;
                        }
                        if target_width as u32 >= self.width {
                            target_width = 0;
                        }
                        //epinntln!(
                        //     "width: {}+{}={}\nheight: {}+{}={}",
                        //     width, w, target_width, height, h, target_height
                        // );
                        if self.get_cell(target_width as u32, target_height as u32)
                            == CellState::Alive
                        {
                            count += 1;
                            //epinntln!("ALIVE Neightbour\n")
                        }
                    }
                }
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
                        if self.get_cell(target_width as u32, target_height as u32)
                            == CellState::Alive
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
    /// calculates and returns the next iteration of the board
    pub fn calc_next_iteration(&self, eb: &EdgeBehavior) -> GoLField {
        let mut ret = GoLField::new(self.width, self.height);
        for h in 0..self.height {
            for w in 0..self.width {
                let al_neigthbours = self.get_alive_neightbour_count(w, h, &eb);
                //epinntln!("w: {}, h:{}, aln: {}\n", w, h, al_neigthbours);
                match al_neigthbours {
                    0...1 => {
                        //Zelle wird nicht belebt
                    }
                    2 => {
                        if self.get_cell(w, h) == CellState::Alive {
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

    fn get_relbyte_and_bits_to_shift(&self, width: u32, height: u32) -> (usize, u8) {
        assert!(width < self.width);
        let bit: u64 = (u64::from(self.width) * u64::from(height)) + u64::from(width);
        let byte_no = (bit as f64 / 8_f64) as usize;
        assert!(byte_no < self.field_data.len());
        let bit_to_shift: u8 = 7 - (bit % 8) as u8;
        assert!(bit_to_shift < 8);
        (byte_no, bit_to_shift)
    }
    /// creates a new gameboard with the given width and height
    pub fn new(width: u32, height: u32) -> GoLField {
        let needed_bytes = (u64::from(width) * u64::from(height)) as f64 / 8_f64;
        //epinntln!("bytes needed = {}", needed_bytes);
        let needed_bytes = needed_bytes.ceil();
        let field = vec![0_u8; needed_bytes as usize];

        GoLField {
            field_data: field,
            width,
            height,
        }
    }

    /// Gets the state of a given cell
    pub fn get_cell(&self, width: u32, height: u32) -> CellState {
        let bit_stuff = self.get_relbyte_and_bits_to_shift(width, height);
        let relevant_byte = self.field_data[bit_stuff.0];

        if relevant_byte & (0x01 << bit_stuff.1) != 0x00 {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }

    /// Sets the state of a given cell to alive
    pub fn set_cell_alive(&mut self, width: u32, height: u32) {
        let bit_stuff = self.get_relbyte_and_bits_to_shift(width, height);

        if let Some(relevant_byte) = self.field_data.get_mut(bit_stuff.0) {
            *relevant_byte |= 0x1 << bit_stuff.1;
        } else {
            unreachable!()
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
    fn iter_simple_deadcell() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(1, 1);
        assert_eq!(field.field_data, &[8_u8, 0_u8]);
        let new_field = field.calc_next_iteration(&super::EdgeBehavior::DeadCells);
        assert_eq!(new_field.field_data, &[0_u8, 0_u8]);
    }

    #[test]
    fn iter_simple_static_block_deadcell() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(0, 0);
        field.set_cell_alive(0, 1);
        field.set_cell_alive(1, 1);
        field.set_cell_alive(1, 0);
        assert_eq!(field.field_data, &[216_u8, 0_u8]);
        let new_field = field.calc_next_iteration(&super::EdgeBehavior::DeadCells);
        assert_eq!(new_field.field_data, &[216_u8, 0_u8]);
    }

    #[test]
    fn iter_blink_deadcell() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(1, 0); //010
        field.set_cell_alive(1, 1); //010
        field.set_cell_alive(1, 2); //010
        assert_eq!(field.field_data, &[73_u8, 0_u8]); //0100_1001_0000_0000
        let new_field = field.calc_next_iteration(&super::EdgeBehavior::DeadCells);
        /*
        000
        111
        000

        0001_1100_0000_0000
         */
        assert_eq!(new_field.field_data, &[28_u8, 0_u8]);
    }

    #[test]
    fn corpus_static_block_over_edge() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(0, 0);
        field.set_cell_alive(2, 0);
        field.set_cell_alive(0, 2);
        field.set_cell_alive(2, 2);
        /*
        101
        000
        101
        1010_0010_1000_0000
        */

        assert_eq!(field.field_data, &[0xA2, 0x80]);

        let new_field = field.calc_next_iteration(&super::EdgeBehavior::Wrapping);
        assert_eq!(new_field.field_data, &[0xA2, 0x80]);
    }

    #[test]
    fn iter_simple_corpus() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(1, 1);
        assert_eq!(field.field_data, &[8_u8, 0_u8]);
        let new_field = field.calc_next_iteration(&super::EdgeBehavior::Wrapping);
        assert_eq!(new_field.field_data, &[0_u8, 0_u8]);
    }

    #[test]
    fn iter_simple_static_block_corpus() {
        let mut field = super::GoLField::new(3, 3);
        field.set_cell_alive(0, 0);
        field.set_cell_alive(0, 1);
        field.set_cell_alive(1, 1);
        field.set_cell_alive(1, 0);
        assert_eq!(field.field_data, &[216_u8, 0_u8]);
        let new_field = field.calc_next_iteration(&super::EdgeBehavior::Wrapping);
        assert_eq!(new_field.field_data, &[216_u8, 0_u8]);
    }

    #[test]
    fn iter_blink_corpus() {
        let mut field = super::GoLField::new(4, 4);
        field.set_cell_alive(1, 0);
        field.set_cell_alive(1, 1);
        field.set_cell_alive(1, 3);
        assert_eq!(field.field_data, &[68_u8, 4_u8]); //0100_1001_0000_0000

        let new_field = field.calc_next_iteration(&super::EdgeBehavior::Wrapping);
        assert_eq!(new_field.field_data, &[224_u8, 0_u8]);

        let new_field = new_field.calc_next_iteration(&super::EdgeBehavior::Wrapping);
        assert_eq!(new_field.field_data, &[68_u8, 4_u8]); //0100_1001_0000_0000
    }

}
