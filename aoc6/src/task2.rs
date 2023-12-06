use crate::utils::Race;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let race = Race::parse_part_2("../input/6_1_example_input.txt");

        let posibilities = race.beat_record_possibilities();

        assert_eq!(posibilities, 71503);
    }
}

pub fn run_task() {
    let race = Race::parse_part_2("input/6_1_input.txt");

    let posibilities = race.beat_record_possibilities();

    println!("Posibilities is {}", posibilities);
}