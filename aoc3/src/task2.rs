use crate::utils::EnginePart;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let schematic_file = "../input/3_1_example_input.txt";

        let engine_parts = EnginePart::parse_all(schematic_file);
        engine_parts.iter().for_each(|ep| println!("{:?}", ep));

        let gears = EnginePart::filter_gears(&engine_parts);
        gears.iter().for_each(|g| println!("{:?}", g));


        let mut sum = 0;
        for (part_1, part_2) in gears {
            sum += part_1.gear_ratio(part_2);
        }
        
        assert_eq!(sum, 467835);
    }
}

pub fn run_task() {
    let schematic_file = "input/3_1_input.txt";

    let engine_parts = EnginePart::parse_all(schematic_file);

    let gears = EnginePart::filter_gears(&engine_parts);


    let mut sum = 0;
    for (part_1, part_2) in gears {
        sum += part_1.gear_ratio(part_2);
    }
    
    println!("Sum is {}", sum);
}