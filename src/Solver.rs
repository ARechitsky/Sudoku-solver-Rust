use constants::SIZE;
use field::Value;
use field::Field;

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
}