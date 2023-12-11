use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    y: usize,
    x: usize
}

pub struct SkyImage {
    map: Vec<Vec<char>>
}

static GALAXY: char = '#';
static EMPTY: char = '.';

impl SkyImage {
    // fn expand_universe(map: &mut Vec<Vec<char>>) {
    //     // Rows
    //     let mut added_rows = 0;
    //     for row in 0..map.len() {
    //         let line = &map[row + added_rows];
    //         if line.iter().all(|c| *c == EMPTY) {
    //             map.insert(row + added_rows, line.clone());
    //             added_rows += 1;
    //         }
    //     }

    //     // Columns
    //     let cols = map.first().unwrap().len();
    //     let rows = map.len();
    //     let mut added_cols = 0;
    //     for col in 0..cols {
    //         if map.iter().map(|row| row[col + added_cols]).all(|c| c == EMPTY) {
    //             for row in 0..rows {
    //                 map[row].insert(col + added_cols, EMPTY);
    //             }
    //             added_cols += 1;
    //         }
    //     }
    // }

    fn is_empty_row(&self, row: usize) -> bool {
        return self.map[row].iter().all(|c| *c == EMPTY)
    }

    fn is_empty_col(&self, col: usize) -> bool {
        return self.map.iter().map(|row| row[col]).all(|c| c == EMPTY)
    }

    fn find_galaxies(&self, expansion_factor: usize) -> Vec<Point> {
        let mut galaxies= Vec::new();

        let mut empty_rows = 0;
        for row in 0..self.map.len() {
            if self.is_empty_row(row) {
                empty_rows += 1;
                continue;
            }

            let mut empty_cols = 0;
            for col in 0..self.map.first().unwrap().len() {
                if self.is_empty_col(col) {
                    empty_cols += 1;
                    continue;
                }

                if self.map[row][col] == GALAXY {
                    galaxies.push(Point { y: row - empty_rows + empty_rows * expansion_factor, x: col - empty_cols + empty_cols * expansion_factor })
                }
            }
        }

        return galaxies;
    }

    fn find_shortest_path(p1: &Point, p2: &Point) -> usize {
        // Aka Manhattan distance
        ((p1.y as isize - (p2.y as isize)).abs() + (p1.x as isize - (p2.x as isize)).abs()) as usize
    }

    pub fn find_shortest_paths(&self, expansion_factor: usize) -> HashMap<(Point, Point), usize> {
        let galaxies = self.find_galaxies(expansion_factor);

        // Loop through pairs
        let mut res = HashMap::new();
        for (i, p1) in galaxies.iter().enumerate() {
            for p2 in galaxies[i..].iter() {
                let shortest_path = Self::find_shortest_path(p1, p2);
                res.insert((*p1, *p2), shortest_path);
            }
        }

        return res;
    }

    pub fn print(&self) {
        for line in self.map.iter() {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn parse(file: &str) -> SkyImage {
        let lines = aoc_helper::read_lines(file);
        let mut map = Vec::new();

        for line in lines {
            let line_vec: Vec<char> = line.chars().collect();
            map.push(line_vec);
        }

        //Self::expand_universe(&mut map);

        SkyImage { map: map }
    }
}