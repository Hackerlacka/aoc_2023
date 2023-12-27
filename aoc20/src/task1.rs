use crate::utils::ElectronicMap;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let button_presses = 1000;

        // Example 1
        let mut map_1 = ElectronicMap::parse("../input/20_1_example_input_1.txt");
        let (low_pulses_1, high_pulses_1) = map_1.determine_pulses_sent(button_presses);
        assert_eq!((low_pulses_1, high_pulses_1), (8000, 4000));

        let res_1 = low_pulses_1 * high_pulses_1;
        assert_eq!(res_1, 32000000);

        // Example 2
        let mut map_2 = ElectronicMap::parse("../input/20_1_example_input_2.txt");
        let (low_pulses_2, high_pulses_2) = map_2.determine_pulses_sent(button_presses);
        assert_eq!((low_pulses_2, high_pulses_2), (4250, 2750));

        let res_2 = low_pulses_2 * high_pulses_2;
        assert_eq!(res_2, 11687500);
    }
}

pub fn run_task() {
    let button_presses = 1000;

    let mut map = ElectronicMap::parse("input/20_1_input.txt");
    let (low_pulses, high_pulses) = map.determine_pulses_sent(button_presses);

    let res = low_pulses * high_pulses;
    println!("Res is: {}", res);
}