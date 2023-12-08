use crate::utils::Network;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let network = Network::parse("../input/8_2_example_input.txt");

        let res = network.steps_to_all_z();

        assert_eq!(res, 6);
    }
}

pub fn run_task() {
    let network = Network::parse("input/8_1_input.txt");

    let res = network.steps_to_all_z();

    println!("Res: {}", res);
}