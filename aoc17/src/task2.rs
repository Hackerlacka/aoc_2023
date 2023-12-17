use crate::utils::HeatLossMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut heat_loss_map_1 = HeatLossMap::parse("../input/17_1_example_input.txt");
        let heat_loss_lowest_1 = heat_loss_map_1.find_lowest_heat_loss_path(true);
        assert_eq!(heat_loss_lowest_1, 94);

        println!("--------");

        let mut heat_loss_map_2 = HeatLossMap::parse("../input/17_2_example_input.txt");
        let heat_loss_lowest_2 = heat_loss_map_2.find_lowest_heat_loss_path(true);
        assert_eq!(heat_loss_lowest_2, 71);
    }
}

pub fn run_task() {
    let mut heat_loss_map = HeatLossMap::parse("input/17_1_input.txt");

    let heat_loss_lowest = heat_loss_map.find_lowest_heat_loss_path(true);

    println!("Lowest heat loss (ultra crucible): {}", heat_loss_lowest);
}