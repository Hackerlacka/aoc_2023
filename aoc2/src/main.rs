mod task1;
mod task2;
mod utils;
use std::time::Instant;

fn main() {
    let before = Instant::now();
    task1::run_task();
    println!("Elapsed time: {:.2?}", before.elapsed());

    let before2 = Instant::now();
    task2::run_task();
    println!("Elapsed time: {:.2?}", before2.elapsed());
}
