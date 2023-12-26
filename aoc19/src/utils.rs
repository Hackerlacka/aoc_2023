use std::collections::VecDeque;
use regex::Regex;

enum Category {
    X,
    M,
    A,
    S
}

#[derive(PartialEq)]
enum Comparator {
    LT,
    GT
}

#[derive(Clone, Copy)]
struct MachinePartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64)
}

pub struct MachinePart {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

struct RuleCondition {
    category: Category,
    comparator: Comparator,
    value: u64
}

struct Rule {
    condition: Option<RuleCondition>,
    destination: String
}

struct Workflow {
    name: String,
    rules: Vec<Rule>
}

pub struct ElfSystem {
    workflows: Vec<Workflow>,
    parts: Vec<MachinePart>
}

impl Category {
    fn new(c: char) -> Category {
        match c {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("Cannot parse Category from: {}", c)
        }
    }
}

impl Comparator {
    fn compare_range(&self, category_range: (u64, u64), value: u64) -> (Option<(u64, u64)>, Option<(u64, u64)>) {

        // Comparison: (u64, u64) < u64?
        if *self == Comparator::LT {
            return if value > category_range.1 {
                (Some(category_range), None)
            } else if value == category_range.1 {
                if category_range.1 == category_range.0 {
                    (None, Some(category_range))
                } else {
                    (Some((category_range.0, category_range.1 - 1)), Some((value, value)))
                }
            } else if value < category_range.1 && value > category_range.0 {
                (Some((category_range.0, value - 1)), Some((value, category_range.1)))
            } else if value <= category_range.0 {
                (None, Some(category_range))
            } else {
                panic!("Failed compare_range LT");
            }
        } else { // Comparison: (u64, u64) > u64?
            return if value < category_range.0 {
                (Some(category_range), None)
            } else if value == category_range.0 {
                if category_range.0 == category_range.1 {
                    (None, Some(category_range))
                } else {
                    (Some((category_range.0 + 1, category_range.1)), Some((value, value)))
                }
            } else if value < category_range.1 && value > category_range.0 {
                (Some((value + 1, category_range.1)), Some((category_range.0, value)))
            } else if value >= category_range.1 {
                (None, Some(category_range))
            } else {
                panic!("Failed compare_range GT");
            }
        }
        
    }

    fn compare(&self, category_value: u64, value: u64) -> bool {
        return match self {
            Comparator::LT => category_value < value,
            Comparator::GT => category_value > value
        }
    }

    fn new(c: char) -> Comparator {
        match c {
            '<' => Comparator::LT,
            '>' => Comparator::GT,
            _ => panic!("Could not parse Comparator from: {}", c)
        }
    }
}

impl MachinePartRange {
    fn set_category_range(&mut self, category: &Category, range: (u64, u64)) {
        match category {
            Category::X => self.x = range,
            Category::M => self.m = range,
            Category::A => self.a = range,
            Category::S => self.s = range
        };
    }

    fn get_category_range(&self, category: &Category) -> (u64, u64) {
        return match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s
        };
    }

    fn calculate_distinct_combinations(&self) -> u64 {
        let x_combinations = self.x.1 - self.x.0 + 1;
        let m_combinations = self.m.1 - self.m.0 + 1;
        let a_combinations = self.a.1 - self.a.0 + 1;
        let s_combinations = self.s.1 - self.s.0 + 1;

        return x_combinations * m_combinations * a_combinations * s_combinations;
    }
}

impl MachinePart {
    fn get_category_value(&self, category: &Category) -> u64 {
        return match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s
        };
    }

    fn get_rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }

    fn parse(line: &str) -> MachinePart {
        let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();

        for (_, [x, m, a, s]) in re.captures_iter(line).map(|c| c.extract()) {
            return MachinePart { x: x.parse::<u64>().unwrap(), m: m.parse::<u64>().unwrap(), a: a.parse::<u64>().unwrap(), s: s.parse::<u64>().unwrap() };
        }

        panic!("Unparseable MachinePart: {}", line);
    }
}

impl RuleCondition {
    fn check_condition_range(&self, range: &MachinePartRange) -> (Option<MachinePartRange>, Option<MachinePartRange>) {
        let category_range = range.get_category_range(&self.category);
        let (left, right) = self.comparator.compare_range(category_range, self.value);

        let mut left_machine_range = None;
        if left.is_some() {
            let mut clone = range.clone();
            clone.set_category_range(&self.category, left.unwrap());
            left_machine_range = Some(clone);
        }

        let mut right_machine_range = None;
        if right.is_some() {
            let mut clone = range.clone();
            clone.set_category_range(&self.category, right.unwrap());
            right_machine_range = Some(clone);
        }

        return (left_machine_range, right_machine_range);
    }
    
    fn check_condition(&self, part: &MachinePart) -> bool {
        self.comparator.compare(part.get_category_value(&self.category), self.value)
    }

    fn parse(str: &str) -> RuleCondition {
        let split_char = if str.contains('<') { '<' } else { '>' };
        let mut split = str.split(split_char);

        let category = Category::new(split.next().unwrap().chars().next().unwrap());
        let comparator = Comparator::new(split_char);
        let value = split.next().unwrap().parse::<u64>().unwrap();

        RuleCondition { category: category, comparator: comparator, value: value }
    }
}

impl Rule {
    fn test_range(&self, range: &MachinePartRange) -> (Option<(MachinePartRange, String)>, Option<MachinePartRange>) {
        if self.condition.is_none() {
            return (Some((range.clone(), self.destination.clone())), None);
        }

        // Left part goes to destination, right part goes to next rule
        let (left, right) = self.condition.as_ref().unwrap().check_condition_range(range);
        let mut left_out: Option<(MachinePartRange, String)> = None;
        if left.is_some() {
            left_out = Some((left.unwrap(), self.destination.clone()));
        }
        
        return (left_out, right);
    }

    fn test_part<'a>(&self, part: &'a MachinePart) -> Option<(&'a MachinePart, String)> {
        return if self.condition.is_none() || self.condition.as_ref().unwrap().check_condition(part) {
            Some((part, self.destination.clone()))
        } else {
            None
        };
    }

    fn parse(str: &str) -> Rule {
        let split: Vec<&str> = str.split(":").collect();
        if split.len() == 1 {
            return Rule { condition: None, destination: split[0].to_string() };
        } else if split.len() == 2 {
            return Rule { condition: Some(RuleCondition::parse(split[0])), destination: split[1].to_string() };
        }

        panic!("Unexpected rule split length");
    }
}

impl Workflow {
    fn process_range(&self, range: &MachinePartRange, queue: &mut VecDeque<(MachinePartRange, String)>, accepted: &mut Vec<MachinePartRange>) {
        let mut tmp_range = Some(range.clone());
        for rule in self.rules.iter() {
            if tmp_range.is_none() {
                return;
            }

            let split = rule.test_range(&tmp_range.unwrap());

            if split.0.is_some() {
                let res_unwrapped = split.0.unwrap();

                if res_unwrapped.1 == "A" {
                    accepted.push(res_unwrapped.0);
                } else if res_unwrapped.1 != "R" {
                    queue.push_back(res_unwrapped);
                } // Otherwise rejected
            }

            tmp_range = split.1;
        }
    }

    fn process_part<'a>(&self, part: &'a MachinePart, queue: &mut VecDeque<(&'a MachinePart, String)>, accepted: &mut Vec<&'a MachinePart>) {
        for rule in self.rules.iter() {
            let res = rule.test_part(part);
            if res.is_some() {
                let res_unwrapped = res.unwrap();
                
                if res_unwrapped.1 == "A" {
                    accepted.push(res_unwrapped.0);
                } else if res_unwrapped.1 != "R" {
                    queue.push_back(res_unwrapped);
                } // Otherwise rejected

                return;
            }
        }

        panic!("Part was not processed in workflow!");
    }

    fn parse_rules(str: &str) -> Vec<Rule> {
        str.split(',').map(|tmp| Rule::parse(tmp)).collect()
    }
    
    fn parse(line: &str) -> Workflow {
        let re = Regex::new(r"([a-z]+)\{(.*)\}").unwrap();

        for (_, [name, rules]) in re.captures_iter(line).map(|c| c.extract()) {
            
            return Workflow { name: name.to_owned(), rules: Self::parse_rules(rules) };
        }

        panic!("Could not parse Workflow from: {}", line);
    }
}

impl ElfSystem {
    pub fn sum_parts_rating_numbers(parts: &Vec<&MachinePart>) -> u64 {
        parts.iter().map(|part| part.get_rating()).sum()
    }

    fn get_workflow(&self, name: &str) -> &Workflow {
        self.workflows.iter().filter(|workflow| workflow.name == name).next().unwrap()
    }

    pub fn get_distinct_combinations(&self) -> u64 {
        let mut queue = VecDeque::new();
        let start_range = MachinePartRange { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) };
        queue.push_back((start_range, String::from("in")));

        // Process ranges until there are no more to process
        let mut accepted_ranges = Vec::new();
        while let Some((range, workflow_str)) = queue.pop_front() {
            let workflow = self.get_workflow(&workflow_str);

            workflow.process_range(&range, &mut queue, &mut accepted_ranges);
        }

        // Calculate distinct combinations
        accepted_ranges.iter().map(|range| range.calculate_distinct_combinations()).sum()
    }

    pub fn get_accepted_parts(&self) -> Vec<&MachinePart> {
        let mut queue = VecDeque::new();

        // Add machine parts to workflow 'in'
        self.parts.iter().for_each(|part| queue.push_back((part, String::from("in"))));

        let mut accepted_parts = Vec::new();
        while let Some(process) = queue.pop_front() {
            let workflow = self.get_workflow(&process.1);
            workflow.process_part(process.0, &mut queue, &mut accepted_parts)
        }

        return accepted_parts;
    }

    pub fn parse(file: &str) -> ElfSystem {
        let lines = aoc_helper::read_lines(file);
        let mut it = lines.into_iter();

        // Parse workflows
        let mut workflows = Vec::new();
        while let Some(line) = it.next() {
            if line.is_empty() {
                break;
            }
            workflows.push(Workflow::parse(&line));
        }

        // Parse machine parts
        let mut parts = Vec::new();
        while let Some(line) = it.next() {
            parts.push(MachinePart::parse(&line));
        }

        ElfSystem { workflows: workflows, parts: parts }
    }
}