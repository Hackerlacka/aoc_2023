use crate::utils::Network;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let network_1 = Network::parse("../input/8_1_example_input_1.txt");
        let network_2 = Network::parse("../input/8_1_example_input_2.txt");

        //println!("Net 1: {:?}", network_1);
        //println!("Net 2: {:?}", network_2);

        let steps_to_z_1 = network_1.steps_to_node("AAA", "ZZZ");
        let steps_to_z_2 = network_2.steps_to_node("AAA", "ZZZ");

        assert_eq!(steps_to_z_1, 2);
        assert_eq!(steps_to_z_2, 6);
    }
}

pub fn run_task() {
    let network = Network::parse("input/8_1_input.txt");

    let steps_to_z = network.steps_to_node("AAA", "ZZZ");

    println!("Steps to ZZZ: {}", steps_to_z);
}