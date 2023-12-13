#[derive(PartialEq)]
struct LineOfReflection {
    horizontal: bool,
    coordinates: (usize, usize)
}

struct Pattern {
    rows: Vec<Vec<char>>
}

pub struct PatternNotes {
    patterns: Vec<Pattern>
}

impl Pattern {
    fn find_reflections(hashes: Vec<u64>) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        for i in 0..hashes.len() - 1 {
            let p1 = &hashes[..i + 1];
            let p2 = &hashes[(i + 1)..];

            let is_equal = p1.iter().rev().zip(p2.iter()).all(|(h1, h2)| h1 == h2);
            if is_equal {
                res.push((i, i + 1));
            }
        }

        return res;
    }

    fn get_horizontal_hashes(&self) -> Vec<u64> {
        let mut hashes = Vec::new();
        let rows = self.rows.len();

        for y in 0..rows {
            let str: String = self.rows[y].iter().collect();
            hashes.push(aoc_helper::hash(str));
        }

        return hashes;
    }

    fn get_vertical_hashes(&self) -> Vec<u64> {
        let mut hashes = Vec::new();
        let cols = self.rows.first().unwrap().len();

        for x in 0..cols {
            let str: String = self.rows.iter().map(|s| s[x]).collect();
            hashes.push(aoc_helper::hash(str));
        }

        return hashes;
    }

    fn flip_char(rows: &mut Vec<Vec<char>>, y: usize, x: usize) {
        if rows[y][x] == '.' {
            rows[y][x] = '#';
        } else {
            rows[y][x] = '.';
        }
    }

    fn fix_smudge(&mut self) -> LineOfReflection {
        // Get initial / baseline reflection
        let mut baseline_reflections = self.find_line_of_reflections();
        if baseline_reflections.len() != 1 {
            panic!("Basline reflection count is not 1: {}", baseline_reflections.len());
        }
        let baseline_reflection = baseline_reflections.remove(0);

        let dimensions = (self.rows.len(), self.rows.first().unwrap().len());
        for y in 0..dimensions.0 {
            for x in 0..dimensions.1 {
                // Change . to # and vice versa
                Self::flip_char(&mut self.rows, y, x);

                // Look for reflections and compare with baseline
                // "The old reflection line won't necessarily continue being valid after the smudge is fixed"
                let mut reflections = self.find_line_of_reflections();
                if reflections.len() == 1 {
                    if !reflections.contains(&baseline_reflection) {
                        // Revert change
                        Self::flip_char(&mut self.rows, y, x);

                        return reflections.remove(0);
                    }
                } else if reflections.len() == 2 {
                    if !reflections.contains(&baseline_reflection) {
                        panic!("Expected baseline reflection in reflections");
                    }
                    for _ in 0..reflections.len() {
                        let reflection = reflections.remove(0);
                        if reflection != baseline_reflection {
                            return reflection;
                        }
                    }
                } else if reflections.len() > 2 {
                    panic!("Found more that two reflections: {}", reflections.len());
                }

                // Revert change
                Self::flip_char(&mut self.rows, y, x);
            }
        }

        panic!("fix_smudge() found no neww line of reflection!");

    }

    fn find_line_of_reflections(&self) -> Vec<LineOfReflection> {
        let mut lines = Vec::new();

        // Horizontal
        let horizontal_hashes = self.get_horizontal_hashes();
        let mut reflections = Self::find_reflections(horizontal_hashes);
        for _ in 0..reflections.len() {
            let coordinates = reflections.remove(0);
            lines.push(LineOfReflection { horizontal: true, coordinates: coordinates });
        }

        // Vertical
        let vertical_hashes = self.get_vertical_hashes();
        let mut reflections = Self::find_reflections(vertical_hashes);
        for _ in 0..reflections.len() {
            let coordinates = reflections.remove(0);
            lines.push(LineOfReflection { horizontal: false, coordinates: coordinates });
        }
        
        return lines;
    }

    fn parse(lines: Vec<&String>) -> Pattern {
        let rows = lines.iter().map(|line| line.chars().collect()).collect();

        Pattern { rows: rows }
    }
}

impl PatternNotes {
    fn summarize(line_of_reflections: &Vec<LineOfReflection>) -> usize {
        let sum = line_of_reflections.iter().map(|reflection| {
            if !reflection.horizontal {
                // Compensate for index starting at 0
                reflection.coordinates.0 + 1 
            } else {
                // Compensate for index starting at 0
                (reflection.coordinates.0 + 1) * 100
            }
        }).sum();

        return sum;
    }

    pub fn fix_smudges_and_summarize(&mut self) -> usize {
        let line_of_reflections: Vec<LineOfReflection> = self.patterns.iter_mut().map(|pattern| pattern.fix_smudge()).collect();

        Self::summarize(&line_of_reflections)
    }

    pub fn summarize_notes(&self) -> usize {
        let line_of_reflections: Vec<LineOfReflection> = self.patterns.iter().map(|p| {
            let mut tmp_line_of_reflections = p.find_line_of_reflections();

            if tmp_line_of_reflections.len() != 1 {
                panic!("Found more that one line of reflection: {}", tmp_line_of_reflections.len());
            }

            tmp_line_of_reflections.remove(0)
        }).collect();
    
        Self::summarize(&line_of_reflections)
    }

    pub fn parse(file: &str) -> PatternNotes {
        let lines = aoc_helper::read_lines(file);
        let mut it = lines.iter().peekable();
        let mut grouped_lines = Vec::new();

        let mut patterns = Vec::new();
        while let Some(line) = it.next() {
            if line.is_empty() || it.peek().is_none() {
                patterns.push(Pattern::parse(grouped_lines));
                grouped_lines = Vec::new();
            } else {
                grouped_lines.push(line);
            }
        }

        PatternNotes { patterns: patterns }
    }
}