use crate::utils::WireMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let wire_map = WireMap::parse("../input/25_1_example_input.txt");
        let result = wire_map.divide_into_two_groups();
        assert_eq!(result, 54);
    }
}

pub fn run_task() {
    let wire_map = WireMap::parse("input/25_1_input.txt");
    let result = wire_map.divide_into_two_groups();

    println!("Result is: {}", result);
}