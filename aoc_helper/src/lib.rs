use std::time::Instant;

/// Benchmark a function
/// 
/// The result is automatically printed
pub fn benchmark(fun: fn() -> ()) {
    let before = Instant::now();
    fun();
    println!("Elapsed time: {:.2?}", before.elapsed());
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
