use constants::SIZE;

#[derive(Copy, Clone, PartialEq)]
pub enum Value {
    None,
    No,
    Yes
}

pub struct Field {
    pub data: [[[Value; SIZE]; SIZE]; SIZE]
}

impl Field {
    pub fn print(self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                let mut val = 0;
                for k in 0..SIZE {
                    if self.data[i][j][k]==Value::Yes { val = k + 1 }
                }
                print!("{} ", val)
            }
            println!("")
        }
    }
}