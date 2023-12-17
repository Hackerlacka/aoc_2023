use crate::utils::HeatLossMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut heat_loss_map = HeatLossMap::parse("../input/17_1_example_input.txt");

        let heat_loss_lowest = heat_loss_map.find_lowest_heat_loss_path(false);

        assert_eq!(heat_loss_lowest, 102);
    }
}

pub fn run_task() {
    let mut heat_loss_map = HeatLossMap::parse("input/17_1_input.txt");

    let heat_loss_lowest = heat_loss_map.find_lowest_heat_loss_path(false);

    println!("Lowest heat loss: {}", heat_loss_lowest);
}