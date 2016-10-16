extern crate time;

pub mod constants;
pub mod field;
pub mod solver;
pub mod strategies;
pub mod timer;

use field::Value;
use timer::Timer;

fn main() {
    let str_rep = [
        "000030002004000009000051040040320090002108300050046010030560000500000400200090000",
        "000040700031500006600037090000093025000000000950680000080310009400008630003060000",
        "006000137900600508025381009102860700600053900390702850009146075460030091013097000",
        "020003000405209000070008532102000000080000020000000304913500040000906803000300070",
        "482700100000043000300100900031000005090000030800000670003004006000370000005009312",
    ];
    let mut timer: Timer = Default::default();
    timer.start();
    for rep in str_rep.into_iter() {
        test(rep.to_string())
    }
    timer.stop();
    println!("Avg time: {:.3} seconds", timer.get_time() / (str_rep.len() as f64))
}

fn test(str_rep: String) {
    let solver = solver::Solver::new(str_rep);
    match solver {
        Ok(mut solver) => {
            println!("");
            println!("=================");
            println!("");
            solver.field.print();
            println!("");
            solver.solve();
            solver.field.print();
            /*for i in 0..SIZE {
                for j in 0..SIZE {
                    for k in 0..SIZE {
                        print!("{}", disp(solver.field.data[i][j][k]));
                    }
                    println!("");
                }
                println!("");
            }*/
            println!("");
        },
        Err(s) => {
            println!("{} ", s);
        }
    }
}

fn disp(val: Value) -> i32 {
    match val {
        Value::Yes => 1,
        Value::No => 2,
        _ => 0
    }
}
