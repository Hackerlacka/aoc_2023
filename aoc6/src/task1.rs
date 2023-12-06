use crate::utils::Race;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let races = Race::parse_all("../input/6_1_example_input.txt");

        let posibilities: Vec<u64> = races.iter().map(|race| race.beat_record_possibilities()).collect();

        assert_eq!(posibilities, vec![4, 8, 9]);

        let mut res = 1;
        for posibility in posibilities {
            res *= posibility;
        }

        assert_eq!(res, 288);
    }
}

pub fn run_task() {
    let races = Race::parse_all("input/6_1_input.txt");

    let posibilities: Vec<u64> = races.iter().map(|race| race.beat_record_possibilities()).collect();

    let mut res = 1;
    for posibility in posibilities {
        res *= posibility;
    }

    println!("Res is {}", res);
}