use crate::utils::read_calibration_vals;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_2_example_input() {
        let calibration_vals = read_calibration_vals("../input/1_2_example_input.txt", true);

        assert_eq!(calibration_vals, vec![29, 83, 13, 24, 42, 14, 76]);

        let calibration_vals_sum: i32 = calibration_vals.iter().sum();

        assert_eq!(calibration_vals_sum, 281);
    }
}

pub fn run_task() {
    let calibration_vals = read_calibration_vals("input/1_2_input.txt", true);
    let calibration_vals_sum: i32 = calibration_vals.iter().sum();

    println!("Calibration values sum are {}", calibration_vals_sum);
}