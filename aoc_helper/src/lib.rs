use std::collections::VecDeque;
use std::time::Instant;
use std::fs::read_to_string;

/// Benchmark a function
/// 
/// The result is automatically printed
pub fn benchmark(fun: fn() -> ()) {
    let before = Instant::now();
    fun();
    println!("Elapsed time: {:.2?}", before.elapsed());
}

/// Get all lines in a file
pub fn read_lines(file: &str) -> Vec<String> {
    read_to_string(file).unwrap().lines().map(String::from).collect()
}

/// Get all lines in a file (as double ended queue)
pub fn read_lines_deque(file: &str) -> VecDeque<String> {
    read_to_string(file).unwrap().lines().map(String::from).collect()
}

// TODO: Remove?
pub fn get_input_path(day: u32, task: u32, example: bool) -> String {
    return format!("input/{}_{}_{}input.txt", day, task, if example {"example_"} else {""})
}


// TODO: Remove
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
