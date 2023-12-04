pub struct ScratchCard {
    winning_nbrs: Vec<u32>,
    nbrs: Vec<u32>,
}

impl ScratchCard {
    fn get_nbrs_union(&self) -> Vec<u32> {
        let mut res = Vec::new();

        // TODO: what if duplicates??
        for a in self.winning_nbrs.iter() {
            for b in self.nbrs.iter() {
                if *a == *b {
                    res.push(*a);
                }
            }
        }

        return res;
    }

    pub fn calculate_score(&self) -> u32 {
        let union = self.get_nbrs_union();

        if union.len() == 0 {
            return 0;
        }

        
        let len_as_u32: u32 = union.len().try_into().unwrap();
        // TryInto::<u32>::try_into(union.len()).unwrap(); // Single line conversion

        return 2_u32.pow(len_as_u32 - 1);
    }

    pub fn calculate_wins(&self) -> u32 {
        let union = self.get_nbrs_union();

        union.len().try_into().unwrap()
    }

    fn parse_nbrs(nbrs_str: &str) -> Vec<u32> {
        let tmp_str = nbrs_str.trim().replace("  ", " ");
        tmp_str.split(" ").map(|ns| ns.parse::<u32>().unwrap()).collect()
    }

    fn parse(line: &str) -> ScratchCard {
        let all_nbrs_str = line.split(": ").last().unwrap();
        let mut split_nbrs_str = all_nbrs_str.split(" | ");

        let winning_nbrs = Self::parse_nbrs(split_nbrs_str.next().unwrap());
        let nbrs = Self::parse_nbrs(split_nbrs_str.next().unwrap());

        ScratchCard { winning_nbrs: winning_nbrs, nbrs: nbrs }
    }

    pub fn parseAll(file: &str) -> Vec<ScratchCard> {
        let lines = aoc_helper::read_lines(file);
        let mut res = Vec::new();

        for line in lines.iter() {
            res.push(Self::parse(line));
        }

        return res;
    }
}