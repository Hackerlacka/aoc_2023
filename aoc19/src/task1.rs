use crate::utils::{ElfSystem, MachinePart};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_input() {
        let elf_system = ElfSystem::parse("../input/19_1_example_input.txt");
        let accepted_parts = elf_system.get_accepted_parts();
        let sum_rating = ElfSystem::sum_parts_rating_numbers(&accepted_parts);

        assert_eq!(sum_rating, 19114);
    }
}

pub fn run_task() {
    let elf_system = ElfSystem::parse("input/19_1_input.txt");
    let accepted_parts = elf_system.get_accepted_parts();
    let sum_rating = ElfSystem::sum_parts_rating_numbers(&accepted_parts);

    println!("Sum rating is: {}", sum_rating);
}