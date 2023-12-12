#[derive(Debug)]
struct Spring {
    row: Vec<char>,
    group_sizes: Vec<usize>
}

pub struct SpringMap {
    springs: Vec<Spring>
}

impl Spring {
    fn is_valid(&self, row: Vec<char>) -> bool {
        let s: String = row.iter().collect();
        let splits: Vec<&str> = s.split('.').collect();
        let splits_filtered: Vec<&&str> = splits.iter().filter(|item| !item.is_empty()).collect();

        if splits_filtered.len() == 0 {
            return false;
        }

        if splits_filtered.len() != self.group_sizes.len() {
            return false;
        }

        for (split, exp_group_size) in splits_filtered.iter().zip(self.group_sizes.iter()) {
            if !split.chars().all(|c| c == '#') {
                panic!("Split contains other chars than '#'");
            }

            if split.len() != *exp_group_size {
                return false;
            }
        }

        //println!("Actual row: {:?}, test_row: {:?} is ok", self.row, row);

        return true;
    }

    fn calculate_arrangements(&self) -> usize {
        //println!("Calculating for row {:?}", self.row);

        // Work from left to right in a brute force manner
        let mut res = 0;
        let unknown_count = self.row.iter().filter(|c| **c == '?').count();
        let combinations: usize = 2_usize.pow(unknown_count as u32); // E.g. ??? == 2^3 == 8

        for i in 0..combinations {
            let mut row_copy = self.row.clone();

            // 0 ...
            // 1 #..
            // 2 .#.
            // 3 ##.
            // 4 ..#

            let mut unknown_encounterd_count = 0; // 0..3
            for j in 0..row_copy.len() {
                if row_copy[j] == '?' {
                    // Put # or .
                    if 2_usize.pow(unknown_encounterd_count) & i > 0 {
                        row_copy[j] = '#';
                    } else {
                        row_copy[j] = '.';
                    }

                    unknown_encounterd_count += 1; 
                }
            }

            if self.is_valid(row_copy) {
                res += 1;
            }
        }

        return res;
    }

    fn parse(line: &str) -> Spring {
        let mut split_line = line.split(" ");
        let row = split_line.next().unwrap().chars().collect();
        let group_sizes: Vec<usize> = split_line.next().unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        
        Spring { row: row, group_sizes: group_sizes }
    }
}

impl SpringMap {
    pub fn print(&self) {
        for spring in self.springs.iter() {
            println!("{:?}", spring);
        }
    }

    pub fn calculate_arrangements(&self) -> Vec<usize> {
        self.springs.iter().map(|spring| spring.calculate_arrangements()).collect()
    }

    // TODO: remove?
    // pub fn unfold(&mut self) {
    //     self.springs.iter_mut().for_each(|spring| spring.unfold());
    // }

    pub fn parse(file: &str) -> SpringMap {
        let lines = aoc_helper::read_lines(file);
        let springs = lines.iter().map(|line| Spring::parse(line)).collect();
        
        SpringMap { springs: springs }
    }
}