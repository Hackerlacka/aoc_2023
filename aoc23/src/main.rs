mod task1;
mod task2;
mod utils;
mod utils2;

fn main() {
    aoc_helper::benchmark(task1::run_task);
    aoc_helper::benchmark(task2::run_task);
}
