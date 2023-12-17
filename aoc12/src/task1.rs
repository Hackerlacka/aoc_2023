use crate::utils::SpringMap;

#[cfg(test)]
mod tests {
    use crate::utils::SpringMap;

    use super::*;

    #[test]
    fn test_example_input() {
        let spring_map = SpringMap::parse("../input/12_1_example_input.txt");
        //spring_map.print();

        let arrangements = spring_map.calculate_arrangements();
        assert_eq!(arrangements, vec![1, 4, 1, 1, 4, 10]);
        
        let sum = arrangements.iter().sum::<usize>();
        assert_eq!(sum, 21);
    }
    
    #[test]
    fn test_input_filtered_3() {
        let spring_map = SpringMap::parse("../input/12_1_input_filtered_3.txt");

        let arrangements = spring_map.calculate_arrangements();
        
        let sum = arrangements.iter().sum::<usize>();
        assert_eq!(sum, 49762);
    }
}

pub fn run_task() {
    let spring_map = SpringMap::parse("input/12_1_input.txt");
    let arrangements = spring_map.calculate_arrangements();
    let sum = arrangements.iter().sum::<usize>();
    println!("Sum is {}", sum);
}