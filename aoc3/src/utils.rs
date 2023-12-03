use std::collections::VecDeque;

#[derive(Debug)]
pub struct UnconfirmedPart {
    number_str: String,
    y: isize,
    start: isize,
    end: isize,
}

#[derive(Debug)]
pub struct EnginePart {
    pub y: isize,
    pub number: u32,
    stars: Vec<Star>
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Star {
    x: isize,
    y: isize,
}

impl UnconfirmedPart {
    pub fn new(c: char, y: isize, i: isize) -> UnconfirmedPart {
        UnconfirmedPart { number_str: c.to_string(), y: y, start: i, end: i }
    }

    pub fn add(&mut self, c: char, i: isize) {
        self.number_str.push(c);
        self.end = i;
    }

    pub fn is_close_by(&self, i: isize) -> bool {
        self.start - i <= 1 && self.end - i >= -1
    }

    pub fn to_engine_part(&self) -> EnginePart {
        EnginePart::new(self.y, self.number_str.parse().unwrap())
    }
}

impl Star {
    fn new(x: isize, y: isize) -> Star {
        Star { x: x, y: y }
    }
}

impl EnginePart {
    fn is_special_char(c: char) -> bool {
        !c.is_digit(10) && c != '.'
    }

    fn is_star(c: char) -> bool {
        c == '*'
    }

    fn remove_old_stars(stars: &mut Vec<Star>, x: isize) {
        stars.retain(|star| {
            star.x - x >= -1
        })
    }

    pub fn new(y: isize, number: u32) -> EnginePart {
        EnginePart { y: y, number: number, stars: Vec::new() }
    }

    pub fn has_matching_stars(&self, other: &EnginePart) -> bool {
        for s1 in &self.stars {
            for s2 in &other.stars {
                if s1 == s2 {
                    return true
                }
            }
        }

        return false;
    }

    pub fn add_stars(&mut self, stars: &Vec<Star>) {
        for star in stars {
            self.stars.push(star.clone());
        }
    }

    pub fn parse_all(schematic_file: &str) -> Vec<EnginePart> {
        let mut engine_parts = Vec::new();
        let mut lines = aoc_helper::read_lines_deque(schematic_file);
        let line_len = lines[0].len();

        // Add lines to front and back to allow for 3 line parallel parsing (only accept engine parts in middle line)
        lines.push_front(".".repeat(line_len));
        lines.push_back(".".repeat(line_len));

        for y in 0..(lines.len() - 2) {
            let mut l1 = lines[y].chars();
            let mut l2 = lines[y + 1].chars().zip(0isize..).peekable();
            let mut l3 = lines[y + 2].chars();
            let mut latest_special_c: Option<isize> = None;
            let mut unconf_part: Option<UnconfirmedPart> = None;
            let mut stars: Vec<Star> = Vec::new();
            let ys: isize = y.try_into().unwrap();

            while let Some((l2_char, x)) = l2.next() {
                let l1_char = l1.next().unwrap();
                let l3_char = l3.next().unwrap();

                // Look for special chars on any line
                if Self::is_special_char(l1_char) || 
                   Self::is_special_char(l2_char) ||
                   Self::is_special_char(l3_char) {
                    latest_special_c = Some(x);
                }

                // Save any stars
                if Self::is_star(l1_char) {
                    stars.push(Star::new(x, ys));
                }
                if Self::is_star(l2_char) {
                    stars.push(Star::new(x, ys + 1));
                }
                if Self::is_star(l3_char) {
                    stars.push(Star::new(x, ys + 2));
                }

                // Conditions middle line char
                if l2_char.is_digit(10) {
                    match unconf_part {
                        None => {
                            unconf_part = Some(UnconfirmedPart::new(l2_char, ys, x));
                            Self::remove_old_stars(&mut stars, x); // That are too far away
                        },
                        Some(_) => unconf_part.as_mut().unwrap().add(l2_char, x),
                    }
                } 
                
                // If not digit or if on last char of line
                if !l2_char.is_digit(10) || l2.peek().is_none() {
                    if latest_special_c.is_some() { // And have seen special char
                        let special_char_idx = latest_special_c.unwrap();
                        if unconf_part.is_some() && unconf_part.as_ref().unwrap().is_close_by(special_char_idx) {
                            let mut engine_part = unconf_part.as_ref().unwrap().to_engine_part();
                            engine_part.add_stars(&stars);

                            engine_parts.push(engine_part);
                        }
                    }

                    unconf_part = None;
                }

                if !l2_char.is_digit(10) {
                    Self::remove_old_stars(&mut stars, x + 1); // That are too far away
                }
            }
        }

        return engine_parts;
    }

    pub fn filter_gears(engine_parts: &Vec<EnginePart>) -> Vec<(&EnginePart, &EnginePart)> {
        let mut gears = Vec::new();

        // Remove all engine parts without stars
        let mut filtered_engine_parts: VecDeque<&EnginePart> = engine_parts.iter().filter(|ep| ep.stars.len() > 0).collect();

        // Loop through gears and find gears with the same stars
        'outer: while let Some(ep) = filtered_engine_parts.pop_front() {
            for ep_other in filtered_engine_parts.iter() {
                if ep.y - ep_other.y > 1 {
                    continue 'outer;
                } else if ep.has_matching_stars(ep_other) { // TODO: need to remove other too or not?
                    gears.push((ep, *ep_other)); 
                    continue 'outer;
                }
            }
        }

        return gears;
    }

    pub fn gear_ratio(&self, other: &EnginePart) -> u32 {
        self.number * other.number
    }
}