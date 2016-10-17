use constants::SIZE;
use field::Value;
use field::Field;
use strategies::*;

pub struct Solver {
    pub field: Field
}

impl Solver {
    pub fn new(str_rep: String) -> Result<Solver, String> {
        let mut data = [[[Value::None; SIZE]; SIZE]; SIZE];
        if str_rep.len() != SIZE * SIZE { return Err("Wrong representation".to_string()) };
        for i in 0..SIZE {
            for j in 0..SIZE {
                let pos = i * SIZE + j;
                let val: usize = match str_rep[pos..pos + 1].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return Err("Wrong representation".to_string())
                };
                if val == 0 { continue; }
                data[i][j][val - 1] = Value::Yes;
            }
        }
        Ok(Solver { field: Field::new(data) })
    }

    pub fn solve(&mut self) { self.field = Solver::internal_solve(&mut self.field) }
    fn internal_solve(field: &mut Field) -> Field {
        let mut moved = true;
        let mut mult_move_group: Option<Vec<Move>> = None;
        while moved {
            moved = false;
            mult_move_group = None;
            for strategy in BaseStrategy::all() {
                for move_group in strategy.find_turns(field).iter() {
                    if move_group.len() == 1 {
                        field.apply_move(&move_group[0]);
                        moved = true;
                    } else if move_group.len() > 1 &&
                        (mult_move_group.clone().map_or(true, |g: Vec<Move>| g.len() > move_group.len())) {
                        mult_move_group = Some(move_group.clone())
                    }
                }
            }
        }
        if field.is_done() { return field.clone() }
        match mult_move_group {
            Some(move_group) => {
                for _move in move_group {
                    let mut new_field: Field = field.clone();
                    new_field.apply_move(&_move);
                    new_field = Solver::internal_solve(&mut new_field);
                    if new_field.is_done() { return new_field }
                }
            },
            None => {}
        }
        field.clone()
    }
}
