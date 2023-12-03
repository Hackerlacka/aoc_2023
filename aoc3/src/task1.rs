use crate::utils::EnginePart;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let schematic_file = "../input/3_1_example_input.txt";
        let engine_parts = EnginePart::parse_all(schematic_file);
        
        // TODO: Could assert each individual engine part number

        let sum: u32 = engine_parts.iter().map(|ep| ep.number).sum();
        assert_eq!(sum, 4361);
    }
}

pub fn run_task() {

}