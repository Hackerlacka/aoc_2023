use crate::utils::Almanac;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let almanac = Almanac::parse("../input/5_1_example_input.txt");

        let res = almanac.part_2();

        assert_eq!(res, 46);
    }
}

pub fn run_task() {
    let almanac = Almanac::parse("input/5_1_input.txt");

    let res = almanac.part_2();

    println!("Res is {}: ", res);
}