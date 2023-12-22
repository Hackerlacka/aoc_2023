mod task1;
mod task2;
mod utils;

fn main() {
    aoc_helper::benchmark(task1::run_task); // 39011 is not correct
                                                // 38188
                                                // 37345 is too low
    aoc_helper::benchmark(task2::run_task);
}
