use constants::SIZE;
use field::*;
use std::vec::Vec;

#[derive(Clone)]
pub struct Move {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub value: Value
}

pub trait BaseStrategy {
    fn find_turns(&self, field: &Field) -> Vec<Vec<Move>>;
}

impl BaseStrategy {
    fn definite_turn(x: usize, y: usize, z: usize, value: Value) -> Vec<Move> {
        vec!(Move {
            x: x,
            y: y,
            z: z,
            value: value
        })
    }

    pub fn all() -> Vec<Box<BaseStrategy>> {
        vec!(Box::new(StrategyCheckSimple), Box::new(StrategyCrossOutSimple))
    }
}

pub struct StrategyCrossOutSimple;

impl BaseStrategy for StrategyCrossOutSimple {
    fn find_turns(&self, field: &Field) -> Vec<Vec<Move>> {
        let mut result = Vec::new();
        for group in CellGroupType::iterator() {
            for group_index in 0..SIZE * SIZE {
                for cell in 0..SIZE {
                    let (x, y, z) = Field::get_cell_in_group(group_index, group, cell);
                    if field.data[x][y][z] == Value::Yes {
                        for other in 0..SIZE {
                            let (new_x, new_y, new_z) = Field::get_cell_in_group(group_index, group, other);
                            if field.data[new_x][new_y][new_z] == Value::None {
                                result.push(BaseStrategy::definite_turn(new_x, new_y, new_z, Value::No))
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

pub struct StrategyCheckSimple;

impl BaseStrategy for StrategyCheckSimple {
    fn find_turns(&self, field: &Field) -> Vec<Vec<Move>> {
        let mut result = Vec::new();
        for group in CellGroupType::iterator() {
            for group_index in 0..SIZE * SIZE {
                let mut moves: Vec<Move> = Vec::new();
                let mut yes_count = 0;
                for other in 0..SIZE {
                    let (new_x, new_y, new_z) = Field::get_cell_in_group(group_index, group, other);
                    match field.data[new_x][new_y][new_z] {
                        Value::None =>
                            moves.push(Move { x: new_x, y: new_y, z: new_z, value: Value::Yes }),
                        Value::Yes =>
                            yes_count += 1,
                        _ => {}
                    }
                }
                if yes_count == 0 && moves.len() > 0 {
                    result.push(moves)
                }
            }
        }
        result
    }
}
