use crate::utils::ElectronicMap;

pub fn run_task() {
    //let button_presses = 100000000;

    let mut map = ElectronicMap::parse("input/20_1_input.txt");

    let presses_required_rx = map.buttons_press_required_for_low_rx();
    println!("Presses required: {}", presses_required_rx);

    // let (low_pulses, high_pulses) = map.determine_pulses_sent(button_presses);

    // let res = low_pulses * high_pulses;
    // println!("Res is: {}", res);
}