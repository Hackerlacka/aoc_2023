
use chrono;

#[derive(Debug)]
struct RangeMap {
    dest_range_start: u64,
    src_range_start: u64,
    range_len: u64,
}


#[derive(Debug)]
pub struct XToYMap {
    range_maps: Vec<RangeMap>
}


#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub maps: Vec<XToYMap>,
}

impl RangeMap {
    pub fn map(&self, val: u64) -> Option<u64> {
        if val >= self.src_range_start && val <= self.src_range_start + self.range_len - 1 {
            let diff = (val - self.src_range_start) as i64; // Should never be negative

            return Some(self.dest_range_start + diff as u64)
        }

        None
    } 

    pub fn parse(line: &str) -> RangeMap {
        let values: Vec<u64> = line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();

        if values.len() != 3 {
            panic!("RangeMap values length is too short!")
        }

        RangeMap { dest_range_start: values[0], src_range_start: values[1], range_len: values[2] }
    }
}

impl XToYMap {
    pub fn map(&self, seed: u64) -> u64 {
        for range_map in self.range_maps.iter() {
            if let Some(res) = range_map.map(seed) {
                return res;
            }
        }

        return seed;
    }

    pub fn parse(lines: &[String]) -> XToYMap {
        let mut range_maps = Vec::new();

        let mut it = lines.iter();
        it.next(); // Skip title string

        while let Some(line) = it.next() {
            range_maps.push(RangeMap::parse(line));
        }

        return XToYMap { range_maps: range_maps }
    }
}

impl Almanac {
    fn convert_seed(&self, seed: u64) -> Vec<u64> {
        let mut res = Vec::new();
        let mut current_val = seed;
        res.push(current_val);

        for map in self.maps.iter() {
            current_val = map.map(current_val);
            res.push(current_val);
        }

        return res;
    }

    pub fn convert_seeds(&self) -> Vec<Vec<u64>> {
        let mut res = Vec::new();

        for seed in self.seeds.iter() {
            res.push(self.convert_seed(*seed));
        }

        return res;
    }

    // Skips generation of some vectors (Total time: 2116.55s -> 114.60s LOL!)
    fn convert_seed_p2_optimized(&self, seed: u64) -> u64 {
        let mut current_val = seed;

        for map in self.maps.iter() {
            current_val = map.map(current_val);
        }

        return current_val;
    }

    fn part_2_combinations(&self) {
        let mut tmp_it = self.seeds.iter().peekable();
        let mut combinations:u64 = 0;

        while tmp_it.peek().is_some() {
            tmp_it.next();
            combinations += tmp_it.next().unwrap();
        }

         //2 387 882 574
        println!("Seeds/combinations: {}", combinations);
    }

    pub fn part_2(&self) -> u64 {
        self.part_2_combinations();
        println!("{:?}", chrono::offset::Local::now());

        let mut it = self.seeds.iter().peekable();
        let mut res = u64::MAX;
        let mut progress:u32 = 0;

        while it.peek().is_some() {
            let seed_start = *it.next().unwrap();
            let seed_range_len = *it.next().unwrap();

            for i in 0..seed_range_len {
                let seed = seed_start + i;
                progress += 1;

                if progress % 100000000 == 0 { // 100 000 000
                    println!("{:?}: Progress: {}", chrono::offset::Local::now(), progress);
                }

                //let tmp_min = *self.convert_seed(seed).last().unwrap();
                let tmp_min = self.convert_seed_p2_optimized(seed);
                if tmp_min < res {
                    res = tmp_min;
                }
            }
        }

        return res;
    }

    pub fn parse(file: &str) -> Almanac {
        let lines = aoc_helper::read_lines(file);
        let mut it = lines.iter().enumerate().peekable();

        // Parse seeds: Nice :D
        let seeds = it.next().unwrap().1.split(": ").last().unwrap().split(" ").map(|s| s.parse::<u64>().unwrap()).collect();

        it.next(); // Skip empty line

        let mut maps = Vec::new();

        // Loop over lines for XToYMaps
        let mut map_start_index = 0;
        let mut map_end_index;
        while let Some((i, line)) = it.next() {
            if line.contains("map") {
                map_start_index = i;
            }
            
            let last_line = it.peek().is_none();
            if line.is_empty() || last_line { // Aka end of a XToYMap
                map_end_index = i;
                if last_line {
                    map_end_index += 1;
                }
                let map_lines = &lines[map_start_index..map_end_index];
                let map = XToYMap::parse(map_lines);
                maps.push(map);
            }
        }

        return Almanac { seeds: seeds, maps: maps }
    }
}