use crate::utils::Contraption;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let mut contraption = Contraption::parse("../input/16_1_example_input.txt");
        let energized_tiles = contraption.energize_tiles();

        assert_eq!(energized_tiles, 46);
    }
}

pub fn run_task() {
    let mut contraption = Contraption::parse("input/16_1_input.txt");
    let energized_tiles = contraption.energize_tiles();

    println!("Energized tiles: {}", energized_tiles);
}