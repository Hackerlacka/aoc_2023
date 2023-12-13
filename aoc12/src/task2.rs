use crate::utils::SpringMap;

#[cfg(test)]
mod tests {
    use crate::utils::SpringMap;

    use super::*;

    #[test]
    fn test_extra_input() {
        let mut spring_map = SpringMap::parse("../input/12_1_example_input_extra.txt");
        spring_map.unfold();

        let arrangements = spring_map.calculate_arrangements_smart();
        assert_eq!(arrangements, vec![506250]);
        
        let sum = arrangements.iter().sum::<usize>();
        assert_eq!(sum, 506250);
    }

    #[test]
    fn test_example_input() {
        let mut spring_map = SpringMap::parse("../input/12_1_example_input.txt");
        spring_map.unfold();

        let arrangements = spring_map.calculate_arrangements_smart();
        assert_eq!(arrangements, vec![1, 16384, 1, 16, 2500, 506250]);
        
        let sum = arrangements.iter().sum::<usize>();
        assert_eq!(sum, 525152);
    }
}

pub fn run_task() {
    let mut spring_map = SpringMap::parse("input/12_1_input.txt");
    spring_map.unfold();

    let arrangements = spring_map.calculate_arrangements_smart();

    let sum = arrangements.iter().sum::<usize>();
    println!("Sum is {}", sum);
}