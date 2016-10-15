use constants::*;
use std::slice::Iter;
use self::CellGroupType::*;


#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    None,
    No,
    Yes
}

pub enum CellGroupType {
    XY,
    XZ,
    YZ,
    SquareZ
}

impl CellGroupType {
    pub fn iterator() -> Iter<'static, CellGroupType> {
        static GROUP_TYPES: [CellGroupType; 4] = [XY, XZ, YZ, SquareZ];
        GROUP_TYPES.into_iter()
    }
}

pub struct Field {
    pub data: [[[Value; SIZE]; SIZE]; SIZE]
}

impl Field {
    pub fn print(&self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                let mut val = 0;
                for k in 0..SIZE {
                    if self.data[i][j][k] == Value::Yes { val = k + 1 }
                }
                print!("{} ", val)
            }
            println!("")
        }
    }

    /// Returns cell with index ```index``` in the same square, that contains ```(x, y)``` cell
    fn get_cell_in_square(x: usize, y: usize, index: usize) -> (usize, usize) {
        let square_x = x / BLOCK_H_SIZE;
        let square_y = y / BLOCK_V_SIZE;
        let x_in_square = index % BLOCK_H_SIZE;
        let y_in_square = index / BLOCK_H_SIZE;
        (square_x * BLOCK_H_SIZE + x_in_square, square_y * BLOCK_V_SIZE + y_in_square)
    }

    /// Returns cell with index ```index``` in the group of type ```group_type```,
    /// that contains ```(x, y, z)``` cell
    pub fn get_cell_in_group(x: usize, y: usize, z: usize,
                             group_type: &CellGroupType, index: usize) -> (usize, usize, usize) {
        match group_type {
            &CellGroupType::XY => (x, y, index),
            &CellGroupType::XZ => (x, index, z),
            &CellGroupType::YZ => (index, y, z),
            &CellGroupType::SquareZ => {
                let (new_x, new_y) = Field::get_cell_in_square(x, y, index);
                (new_x, new_y, z)
            }
        }
    }
}