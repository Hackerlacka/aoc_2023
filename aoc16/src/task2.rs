use crate::utils::Contraption;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut contraption = Contraption::parse("../input/16_1_example_input.txt");
        let max_energized_tiles = contraption.find_max_tiles_energized();

        assert_eq!(max_energized_tiles, 51);
    }
}

pub fn run_task() {
    let mut contraption = Contraption::parse("input/16_1_input.txt");
    let max_energized_tiles = contraption.find_max_tiles_energized();

    println!("Max energized tiles: {}", max_energized_tiles);
}