use crate::utils::HailstoneMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let hailstone_map = HailstoneMap::parse("../input/24_1_example_input.txt");
        let rock_start_pos = hailstone_map.find_rock_start_pos();

        assert_eq!(rock_start_pos, (24, 13, 10));

        let res = rock_start_pos.0 + rock_start_pos.1 + rock_start_pos.2;
        assert_eq!(res, 47);
    }
}

pub fn run_task() {
    let hailstone_map = HailstoneMap::parse("input/24_1_input.txt");
    let rock_start_pos = hailstone_map.find_rock_start_pos();

    let res = rock_start_pos.0 + rock_start_pos.1 + rock_start_pos.2;

    println!("Rock start pos: {:?}", rock_start_pos);
    println!("Res is: {}", res);
}