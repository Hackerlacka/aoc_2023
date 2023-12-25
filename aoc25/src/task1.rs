use crate::utils::WireMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let wire_map = WireMap::parse("../input/25_1_example_input.txt");
        wire_map.print();
    }
}

pub fn run_task() {
    let wire_map = WireMap::parse("input/25_1_input.txt");
        wire_map.print();
}