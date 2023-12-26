use crate::utils::ElfSystem;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let elf_system = ElfSystem::parse("../input/19_1_example_input.txt");
        let distinct_combinations = elf_system.get_distinct_combinations();

        assert_eq!(distinct_combinations, 167409079868000);
    }
}

pub fn run_task() {
    let elf_system = ElfSystem::parse("input/19_1_input.txt");
    let distinct_combinations = elf_system.get_distinct_combinations();

    println!("Distinct combinations: {}", distinct_combinations);
}