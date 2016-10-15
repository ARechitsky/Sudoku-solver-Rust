use constants::SIZE;
use field::Value;
use field::Field;
use strategies::*;

pub struct Solver {
    pub field: Field
}

impl Solver {
    pub fn new(str_rep: String) -> Result<Solver, String> {
        let mut field = Field { data: [[[Value::None; SIZE]; SIZE]; SIZE] };
        if str_rep.len() != SIZE * SIZE { return Err("Wrong representation".to_string()) };
        for i in 0..SIZE {
            for j in 0..SIZE {
                let pos = i * SIZE + j;
                let val: usize = match str_rep[pos..pos + 1].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return Err("Wrong representation".to_string())
                };
                if val == 0 { continue; }
                field.data[i][j][val - 1] = Value::Yes;
            }
        }
        Ok(Solver { field: field })
    }

    fn apply_move(&mut self, _move: &Move) {
        self.field.data[_move.x][_move.y][_move.z] = _move.value;
    }

    pub fn solve(&mut self) {
        let mut moved = true;
        while moved {
            moved = false;
            for strategy in BaseStrategy::all() {
                for move_group in strategy.find_turns(&self.field).iter() {
                    if move_group.len() == 1 {
                        self.apply_move(&move_group[0]);
                        moved = true;
                    }
                }
            }
        }
    }
}