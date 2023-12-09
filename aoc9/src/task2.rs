use crate::utils::Oasis;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let oasis = Oasis::parse_all("../input/9_1_example_input.txt");
        
        let extrapolated_values = oasis.extrapolate_all(true);

        assert_eq!(extrapolated_values, vec![-3, 0, 5]);

        let sum_ext_vals = extrapolated_values.iter().sum::<i64>();
        assert_eq!(sum_ext_vals, 2);
    }
}

pub fn run_task() {
    let oasis = Oasis::parse_all("input/9_1_input.txt");
    let extrapolated_values = oasis.extrapolate_all(true);
    let sum_ext_vals = extrapolated_values.iter().sum::<i64>();
        
    println!("Sum backwards extrapolated values: {}", sum_ext_vals);
}