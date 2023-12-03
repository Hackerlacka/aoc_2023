use std::{fs::read_to_string, ops::{Add, Index}};

// pub struct UnconfirmedPart {
//     number_str: String,
//     start: (usize, usize),
//     end: (usize, usize),
// }

pub struct UnconfirmedPart {
    number_str: String,
    start: isize,
    end: isize,
}


pub struct EnginePart {
    pub number: u32
}

impl UnconfirmedPart {
    // pub fn new(c: char, index: (usize, usize)) -> UnconfirmedPart {
    //     UnconfirmedPart { number_str: c.to_string(), start: index, end: index }
    // }

    // pub fn add(&mut self, c: char, index: (usize, usize)) {
    //     self.number_str.push(c);
    //     self.end = index;
    // }

    pub fn new(c: char, i: usize) -> UnconfirmedPart {
        UnconfirmedPart { number_str: c.to_string(), start: i, end: i }
    }

    pub fn add(&mut self, c: char, i: usize) {
        self.number_str.push(c);
        self.end = i;
    }

    pub fn is_close_by(&self, i: usize) -> bool {
         i as isize
    }
}

impl EnginePart {
    // pub fn parse_all(schematic_file: &str) -> Vec<EnginePart> {
    //     let engine_parts = Vec::new();
    //     let lines: Vec<String> = aoc_helper::read_lines(schematic_file);

    //     for (y, line) in lines.iter().enumerate() {
    //         let mut unconf_part: Option<UnconfirmedPart> = None;

    //         for (x, c) in line.char_indices() {
    //             if c.is_numeric() {
    //                 let index = (x, y);
    //                 match unconf_part {
    //                     None => unconf_part = Some(UnconfirmedPart::new(c, index)),
    //                     Some(_) => unconf_part.as_mut().unwrap().add(c, index),
    //                 }
    //             } else if c == '.' {
    //                 if unconf_part.is_some() {

    //                 }
    //             } else { // Special character

    //             }
    //         }
    //     }

    //     return engine_parts;
    // }

    fn is_special_char(c: char) -> bool {
        !c.is_digit(10) && c != '.'
    }

    pub fn parse_all(schematic_file: &str) -> Vec<EnginePart> {
        let engine_parts = Vec::new();
        let lines: Vec<String> = aoc_helper::read_lines(schematic_file);
        let mut unconf_part: Option<UnconfirmedPart> = None;

        /* TODO: loop through 3 lines in parallel, but only accept engine parts in the
         * middle line. Add start and end line to support this
         */

        for i in 0..lines.len() - 3 {
            let mut l1 = lines[i].chars();
            let mut l2 = lines[i + 1].chars();
            let mut l3 = lines[i + 2].chars();
            let mut latest_special_c: Option<usize> = None;

            for (l2_char, i) in l2.zip(0isize..) {
                if Self::is_special_char(l1.next().unwrap()) || 
                   Self::is_special_char(l2_char) ||
                   Self::is_special_char(l3.next().unwrap()) {
                    latest_special_c = Some(i);
                }

                if l2_char.is_digit(10) {
                    match unconf_part {
                        None => unconf_part = Some(UnconfirmedPart::new(l2_char, i)),
                        Some(_) => unconf_part.as_mut().unwrap().add(l2_char, i),
                    }
                } else { // Is . or special char
                    if latest_special_c.is_some() {
                        let special_char_idx = latest_special_c.unwrap();

                    }
                }

                // TODO: take care of if this is last char on line...
            }
        }

        for (y, line) in lines.iter().enumerate() {
            let mut unconf_part: Option<UnconfirmedPart> = None;

            for (x, c) in line.char_indices() {
                if c.is_numeric() {
                    let index = (x, y);
                    match unconf_part {
                        None => unconf_part = Some(UnconfirmedPart::new(c, index)),
                        Some(_) => unconf_part.as_mut().unwrap().add(c, index),
                    }
                } else if c == '.' {
                    if unconf_part.is_some() {
                        
                    }
                } else { // Special character

                }
            }
        }

        return engine_parts;
    }
}