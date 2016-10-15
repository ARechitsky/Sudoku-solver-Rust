use constants::SIZE;
use field::*;
use std::vec::Vec;

pub struct Move {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub value: Value
}

pub trait BaseStrategy {
    fn find_turns(field: Field) -> Vec<Vec<Move>>;

    fn definite_turn(x: usize, y: usize, z: usize, value: Value) -> Vec<Move> {
        vec!(Move {
            x: x,
            y: y,
            z: z,
            value: value
        })
    }
}

pub struct StrategyCrossOutSimple;

impl BaseStrategy for StrategyCrossOutSimple {
    fn find_turns(field: Field) -> Vec<Vec<Move>> {
        let mut result = Vec::new();
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if field.data[x][y][z] == Value::Yes {
                        for group in CellGroupType::iterator() {
                            for other in 0..SIZE {
                                let (new_x, new_y, new_z) = Field::get_cell_in_group(x, y, z, group, other);
                                if field.data[new_x][new_y][new_z] == Value::None {
                                    result.push(StrategyCrossOutSimple::definite_turn(new_x, new_y, new_z, Value::No))
                                }
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
    fn find_turns(field: Field) -> Vec<Vec<Move>> {
        let mut result = Vec::new();
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    for group in CellGroupType::iterator() {
                        let mut moves: Vec<Move> = Vec::new();
                        let mut no_count = 0;
                        let mut yes_count = 0;
                        for other in 0..SIZE {
                            let (new_x, new_y, new_z) = Field::get_cell_in_group(x, y, z, group, other);
                            match field.data[new_x][new_y][new_z] {
                                Value::None =>
                                    moves.push(Move { x: new_x, y: new_y, z: new_z, value: Value::Yes }),
                                Value::Yes =>
                                    yes_count += 1,
                                Value::No =>
                                    no_count += 1
                            }
                        }
                        if yes_count == 0 && moves.len() > 0 {
                            result.push(moves)
                        }
                    }
                }
            }
        }
        result
    }
}
