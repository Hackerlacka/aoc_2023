use std::collections::{VecDeque, HashMap};
use chrono;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct Spring {
    row: Vec<char>,
    group_sizes: Vec<usize>
}

pub struct SpringMap {
    springs: Vec<Spring>
}

pub struct MtContext {
    arrangements: HashMap<usize, usize>,
    springs: Vec<Spring>,
    in_progress: Vec<usize>,
    i: usize
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

    // fn calculate_arrangements_rec(orig_row: &Vec<char>, row: &mut Vec<char>, row_len: usize, start_i: usize, queue: &mut VecDeque<usize>) -> usize {
    //     let mut arrangements = 0;
        
    //     let group_size = queue.pop_front().unwrap();
    //     let group_sum_plus_dots = queue.iter().sum::<usize>() + group_size + queue.len();

    //     for i in start_i..row_len {
    //         // Check if there are not enough chars left
    //         let remaining_chars = row_len - i;
    //         if remaining_chars < group_sum_plus_dots {
    //             break;
    //         }

    //         // Check if all of the coming group_size chars are not '.' (i.e. they are # or ?)
    //         // TODO: Potential optimization by jumping i past any found dot
    //         let enough_group_chars: bool = row[i..(i + group_size)].iter().all(|c| *c != '.');
    //         if enough_group_chars {
    //             // True if there are no more chars after this group
    //             let no_chars_after_group = remaining_chars == group_size;
    //             // Verify that this "char next to group" is ? or . or does not exist
    //             if (!no_chars_after_group && row[i + group_size] != '#') || no_chars_after_group {
    //                 // Add group (for debug purposes)
    //                 for j in 0..group_size {
    //                     row[i + j] = '#';
    //                 }

    //                 // .??#?.#?#?#?
                    
    //                 // .#?#?.#?#?#?
    //                 // .#?#?.#???#?
    //                 // .??#?.#?#?#?

    //                 if queue.is_empty() && !no_chars_after_group && orig_row[(i + group_size)..].iter().any(|c| *c == '#') {
    //                     // "continue"
    //                 } else if queue.is_empty() { // Valid combination
    //                     arrangements += 1;

    //                     // TODO: for debugging
    //                     Self::print_row(row);
    //                 } else {
    //                     let new_start_i = i + group_size + 1; // + 1 to make space for . between groups
    //                     arrangements += Self::calculate_arrangements_rec(orig_row, row, row_len, new_start_i, queue);
    //                 }

    //                 // Remove group (for debug purposes)
    //                 for j in 0..group_size {
    //                     row[i + j] = orig_row[i + j];
    //                 }
    //             }
    //         }

    //         // Not allowed to leave a hashtag behind
    //         let next_iter_move_past_hashtag = row[i] == '#';
    //         if next_iter_move_past_hashtag {
    //             break;
    //         }
    //     }

    //     queue.push_front(group_size);

    //     return arrangements;
    // }

    // fn calculate_arrangements_smart(&self) -> usize {
    //     let mut queue = VecDeque::new();
    //     self.group_sizes.iter().for_each(|group_size| queue.push_back(*group_size));

    //     return Self::calculate_arrangements_rec(&self.row, &mut self.row.clone(), self.row.len(), 0, &mut queue);
    // }

    fn calculate_arrangements_rec(orig_row: &Vec<char>, row: &mut Vec<char>, row_len: usize, start_i: usize, queue: &mut VecDeque<usize>) -> Vec<(usize, usize)> {      
        let group_size = queue.pop_front().unwrap();
        let group_sum_plus_dots = queue.iter().sum::<usize>() + group_size + queue.len();

        let mut res: Vec<(usize, usize)> = Vec::new();
        let mut res_received: Option<Vec<(usize, usize)>> = None;
        for i in start_i..row_len {
            // Check if there are not enough chars left
            let remaining_chars = row_len - i;
            if remaining_chars < group_sum_plus_dots {
                break;
            }

            // Check if all of the coming group_size chars are not '.' (i.e. they are # or ?)
            // TODO: Potential optimization by jumping i past any found dot
            let enough_group_chars: bool = row[i..(i + group_size)].iter().all(|c| *c != '.');
            if enough_group_chars {
                // True if there are no more chars after this group
                let no_chars_after_group = remaining_chars == group_size;
                // Verify that this "char next to group" is ? or . or does not exist
                if (!no_chars_after_group && row[i + group_size] != '#') || no_chars_after_group {
                    // Add group (TODO: for debug purposes)
                    // for j in 0..group_size {
                    //     row[i + j] = '#';
                    // }
                    
                    if queue.is_empty() && !no_chars_after_group && orig_row[(i + group_size)..].iter().any(|c| *c == '#') {
                        // There shall not be any hashtags at the end when all groups are placed!
                        // "continue"
                    } else if queue.is_empty() { // Valid combination
                        res.push((i, 1));
                    } else {
                        let new_start_i = i + group_size + 1; // + 1 to make space for . between groups
                        
                        // If new_start_i is past previous res_received max i, then we must recalculate this ourselves
                        if res_received.is_none() || res_received.as_ref().unwrap().len() == 0 || new_start_i > res_received.as_ref().unwrap().iter().map(|pos| pos.0).max().unwrap() {  
                            //println!("Rec call for start_i:{} i:{}", start_i, i);          
                            res_received = Some(Self::calculate_arrangements_rec(orig_row, row, row_len, new_start_i, queue));
                        }

                        if res_received.is_some() {
                            let tmp_res_received = res_received.as_ref().unwrap();
                            let mut found_i = false;
                            let mut arrangements = 0;

                            //println!("start_i:{} i:{} : Looking for {} in {:?}", start_i, i, new_start_i, tmp_res_received);
                            for tmp in tmp_res_received {
                                if tmp.0 >= new_start_i {
                                    found_i = true;
                                }

                                if found_i {
                                    arrangements += tmp.1;
                                }
                            }

                            // TODO: for debugging
                            // if queue.len() == 1 {
                            //     print!("Combinations is: {} for: ", arrangements);
                            //     Self::print_row(row);
                            // }

                            if arrangements > 0 {
                                //println!("Adding arrangements {}", arrangements);
                                res.push((i, arrangements));
                            }
                        }
                    }

                    // Remove group (TODO: for debug purposes)
                    // for j in 0..group_size {
                    //     row[i + j] = orig_row[i + j];
                    // }
                }
            }

            // Not allowed to leave a hashtag behind
            let next_iter_move_past_hashtag = row[i] == '#';
            if next_iter_move_past_hashtag {
                break;
            }
        }

        queue.push_front(group_size);

        return res;
    }

    fn calculate_arrangements_smart(&self) -> usize {
        let mut queue = VecDeque::new();
        self.group_sizes.iter().for_each(|group_size| queue.push_back(*group_size));

        let res = Self::calculate_arrangements_rec(&self.row, &mut self.row.clone(), self.row.len(), 0, &mut queue);

        return res.into_iter().map(|tmp| tmp.1).sum();
    }

    fn print_row(row: &Vec<char>) {
        row.iter().for_each(|c| print!("{}", c));
        println!()
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

    pub fn calculate_arrangements_smart_mt(&self) -> Vec<usize> {
        let mut cloned_springs = self.springs.clone();
        cloned_springs.reverse();
        let tmp_ctx = MtContext { arrangements: HashMap::new(), springs: cloned_springs, i: 0, in_progress: Vec::new() };
        let ctx = Arc::new(Mutex::new(tmp_ctx));

        let mut handles = vec![];

        // 14
        for _ in 0..1 {
            let lctx = Arc::clone(&ctx);
            let handle = thread::spawn(move || {
                loop {
                    let spring: Spring;
                    let i: usize;

                    {
                        let mut tmp_lctx = lctx.lock().unwrap();

                        if tmp_lctx.springs.is_empty() {
                            println!("Queue empty, exiting thread");
                            return;
                        }
                        
                        spring = tmp_lctx.springs.pop().unwrap();
                        tmp_lctx.i += 1;
                        i = tmp_lctx.i;
                        //tmp_lctx.in_progress.push(i);
                    }

                    let arrangements = spring.calculate_arrangements_smart();

                    {
                        let mut tmp_lctx = lctx.lock().unwrap();

                        tmp_lctx.arrangements.insert(i, arrangements);

                        println!("{:?}: Completed spring: {}", chrono::offset::Local::now(), i);

                        // let position = tmp_lctx.in_progress.iter().position(|element| *element == i).unwrap();
                        // tmp_lctx.in_progress.remove(position);

                        // if tmp_lctx.springs.is_empty() {
                        //     println!("In progress: {:?}", tmp_lctx.in_progress);
                        // }
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Convert HashMap result into Vec
        let arrangements = &ctx.lock().unwrap().arrangements;
        let mut res = Vec::new();
        for i in 0..arrangements.len() {
            res.push(*arrangements.get(&(i + 1)).unwrap());
        }

        return res;
    }

    pub fn calculate_arrangements(&self) -> Vec<usize> {
        self.springs.iter().map(|spring| {
            //println!("Working on new spring...");
            spring.calculate_arrangements()
        }).collect()
    }

    // TODO: Revert
    pub fn unfold(&mut self) {
        self.springs.iter_mut().for_each(|spring| spring.unfold());
    }

    pub fn parse(file: &str) -> SpringMap {
        let lines = aoc_helper::read_lines(file);
        let springs = lines.iter().map(|line| Spring::parse(line)).collect();
        
        SpringMap { springs: springs }
    }
}