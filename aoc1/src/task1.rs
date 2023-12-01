use crate::utils::read_calibration_vals;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_1_example_input() {
        let calibration_vals = read_calibration_vals("../input/1_1_example_input.txt", false);

        assert_eq!(calibration_vals, vec![12, 38, 15, 77]);

        let calibration_vals_sum: i32 = calibration_vals.iter().sum();

        assert_eq!(calibration_vals_sum, 142);
    }
}

pub fn run_task() {
    let calibration_vals = read_calibration_vals("input/1_1_input.txt", false);
    let calibration_vals_sum: i32 = calibration_vals.iter().sum();

    println!("Calibration values sum are {}", calibration_vals_sum);
}