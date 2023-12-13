use std::collections::VecDeque;
use chrono;

#[derive(Debug)]
struct Spring {
    row: Vec<char>,
    group_sizes: Vec<usize>
}

pub struct SpringMap {
    springs: Vec<Spring>
}

impl Spring {
    fn unfold(&mut self) {
        let mut new_row = Vec::new();
        for _ in 0..5 {
            new_row.append(&mut self.row.clone());
            new_row.push('?');
        }
        new_row.remove(new_row.len() - 1);

        let mut new_group_sizes = Vec::new();
        for _ in 0..5 {
            new_group_sizes.append(&mut self.group_sizes.clone())
        }

        self.row = new_row;
        self.group_sizes = new_group_sizes;
    }

    fn is_valid(row: Vec<char>, group_sizes: &Vec<usize>) -> bool {
        let s: String = row.iter().collect();
        let splits: Vec<&str> = s.split('.').collect();
        let splits_filtered: Vec<&&str> = splits.iter().filter(|item| !item.is_empty()).collect();

        if splits_filtered.len() == 0 {
            return false;
        }

        if splits_filtered.len() != group_sizes.len() {
            return false;
        }

        for (split, exp_group_size) in splits_filtered.iter().zip(group_sizes.iter()) {
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

    fn calculate_arrangements_rec(orig_row: &Vec<char>, row: &mut Vec<char>, row_len: usize, start_i: usize, queue: &mut VecDeque<usize>) -> usize {
        let group_size = queue.pop_front().unwrap();
        let mut sum = 0; // Arrangements

        // TODO: calc queue.sum() + group_size
        // TODO: calc chars left
        let mut group_sum: usize = queue.iter().sum();
        group_sum += group_size;

        let mut chars_left = row_len - start_i;

        for i in start_i..row_len {

            // TODO if sum > chars left
            if group_sum > chars_left {
                break;
            }
            // Decrease chars left
            chars_left -= 1;

            // Cannot continue if there are not enough chars left for the group
            let remaining_chars = row_len - i;
            if remaining_chars < group_size {
                break;
            }

            // If all are group chars (# or ?) it is ok to continue
            // TODO: Optimize with express as 
            //let is_group_chars = row[i..(i + group_size)].iter().all(|c| *c == '#' || *c == '?');
            let is_group_chars = !row[i..(i + group_size)].iter().any(|c| *c == '.');

            if is_group_chars {
                // True if there are no more chars after this group
                let is_last_chars = remaining_chars == group_size;

                // Verify that this "char next to group" is ? or . or does not exist
                // TODO: Optimize with row[i + group_size] != '#'
                //if (remaining_chars >= group_size + 1 && (row[i + group_size] == '?' || row[i + group_size] == '.')) || is_last_chars {
                if (!is_last_chars && row[i + group_size] != '#') || is_last_chars {
                    // Add group (TODO: NOT actually needed)
                    // for j in 0..group_size {
                    //     row[i + j] = '#';
                    // }

                    if queue.is_empty() { // Valid combination
                        //println!("Row {:?}", row);
                        sum += 1;
                    } else {
                        // TODO: Could possible be out of bounds?
                        let new_start_i = i + group_size + 1; // + 1 to make space for . between groups
                        sum += Self::calculate_arrangements_rec(orig_row, row, row_len, new_start_i, queue);
                    }

                    // Remove group (TODO: NOT actually needed)
                    // for j in 0..group_size {
                    //     row[i + j] = orig_row[i + j];
                    // }
                }
            }

            // Not allowed to leave a hashtag behind
            let next_iter_move_past_hashtag = row[i] == '#';
            if next_iter_move_past_hashtag {
                //println!("Moving past outer! {:?}, group_size {}, queue len {}", row, group_size, queue.len());
                break;
            }
        }

        queue.push_front(group_size);

        return sum;
    }

    fn calculate_arrangements_smart(&self) -> usize {
        let mut queue = VecDeque::new();
        self.group_sizes.iter().for_each(|group_size| queue.push_back(*group_size));

        return Self::calculate_arrangements_rec(&self.row, &mut self.row.clone(), self.row.len(), 0, &mut queue);
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

            if Self::is_valid(row_copy, &self.group_sizes) {
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

    pub fn calculate_arrangements_smart(&self) -> Vec<usize> {
        let mut i = 0;
        self.springs.iter().map(|spring| {
            i += 1;
            println!("{:?}", chrono::offset::Local::now());
            println!("Working on spring {}/{}...", i, self.springs.len());
            spring.calculate_arrangements_smart()
        }).collect()
    }

    pub fn calculate_arrangements(&self) -> Vec<usize> {
        self.springs.iter().map(|spring| {
            println!("Working on new spring...");
            spring.calculate_arrangements()
        }).collect()
    }

    pub fn unfold(&mut self) {
        self.springs.iter_mut().for_each(|spring| spring.unfold());
    }

    pub fn parse(file: &str) -> SpringMap {
        let lines = aoc_helper::read_lines(file);
        let springs = lines.iter().map(|line| Spring::parse(line)).collect();
        
        SpringMap { springs: springs }
    }
}