#[derive(Clone)]
struct HistoryValue {
    values: Vec<i64>
}

pub struct Oasis {
    history_values: Vec<HistoryValue>
}

impl HistoryValue {
    fn reverse(&mut self) {
        self.values.reverse();
    }

    fn extrapolate_rec(values: &Vec<i64>) -> i64 {
        let mut it: std::iter::Peekable<std::slice::Iter<'_, i64>> = values.iter().peekable();
        
        // Calculate differences
        let mut differences = Vec::new();
        while let Some(value) = it.next() {
            let next_value = it.peek();
            if next_value.is_none() {
                break
            }

            let diff = *next_value.unwrap() - value;
                differences.push(diff);
        }

        // Check if we are on the lowest level
        if differences.iter().all(|diff| *diff == 0) {
            return *values.last().unwrap();
        } else {
            return *values.last().unwrap() + Self::extrapolate_rec(&differences);
        }
    }

    fn extrapolate(&self) -> i64 {
        return Self::extrapolate_rec(&self.values);
    }

    fn parse(line: &str) -> HistoryValue {
        let values = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

        HistoryValue { values: values }
    }
}

impl Oasis {
    pub fn extrapolate_all<'a>(&self, backwards: bool) -> Vec<i64> {
        let mut history_values = &self.history_values;
        let mut tmp;

        // Only clone in the backwards case (looks a bit ugly though :D)
        if backwards {
            tmp = self.history_values.clone();
            tmp.iter_mut().for_each(|hv| hv.reverse());
            history_values = &tmp;
        }

        history_values.iter().map(|hv| hv.extrapolate()).collect()
    } 

    pub fn parse_all(file: &str) -> Oasis {
        let lines = aoc_helper::read_lines(file);
        let history_values = lines.iter().map(|line| HistoryValue::parse(line)).collect();

        Oasis { history_values: history_values }
    }
}