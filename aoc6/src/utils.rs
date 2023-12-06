pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn distance_from_init_speed(gas_time: u64, max_time: u64) -> u64 {
        let velocity = gas_time;
        let time_remain = max_time - gas_time;

        return time_remain * velocity;
    }

    pub fn beat_record_possibilities(&self) -> u64 {
        let mut start_beat_time = 0;

        for gas_time in 0..self.time + 1 {
            let distance = Self::distance_from_init_speed(gas_time, self.time);

            if distance > self.distance {
                start_beat_time = gas_time;
                break;
            }
        }

        let mut end_beat_time = 0;
        for gas_time in (0..self.time).rev() { // Don't need to include 0 really
            let distance = Self::distance_from_init_speed(gas_time, self.time);

            if distance > self.distance {
                end_beat_time = gas_time;
                break;
            }
        }


        return end_beat_time - start_beat_time + 1
    }

    fn parse_numbers(line: &str) -> Vec<u64> {
        let mut res = Vec::new();

        line.split(":").last().unwrap().split(" ").for_each(|s| {
            if !s.trim().is_empty() {
                res.push(s.parse::<u64>().unwrap());
            }
        });

        return res;
    }

    fn parse_kerning_number(line: &str) -> u64 {
        let split_line = line.split(":").last().unwrap();
        let trimmed_line = split_line.replace(" ", "");

        println!("Trimmed line is: {}", trimmed_line);

        trimmed_line.parse::<u64>().unwrap()
    }

    pub fn parse_part_2(file: &str) -> Race {
        let lines = aoc_helper::read_lines(file);
        let mut it = lines.iter();

        let time = Self::parse_kerning_number(it.next().unwrap());
        let distance = Self::parse_kerning_number(it.next().unwrap());

        return Race { time: time, distance: distance };
    }

    pub fn parse_all(file: &str) -> Vec<Race> {
        let lines = aoc_helper::read_lines(file);
        let mut it = lines.iter();

        let times = Self::parse_numbers(it.next().unwrap());
        let distances: Vec<u64> = Self::parse_numbers(it.next().unwrap());

        println!("Times: {:?}, distances: {:?}", times, distances);

        let mut races = Vec::new();
        for (time, distance) in times.iter().zip(distances.iter()) {
            races.push(Race { time: *time, distance: *distance })
        }

        return races;
    }
}