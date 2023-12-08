use std::collections::HashMap;
use regex::Regex;
use chrono;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;

#[derive(Debug)]
struct Node {
    pub label: String,
    pub left: String,
    pub right: String
}

#[derive(Debug)]
pub struct Network {
    instructions: String,
    nodes: HashMap<String, Node>
}

impl Network {
    fn open_appendable_files(name: &str, count: u32) -> Vec<File> {
        let mut files = Vec::new();

        for i in 0..count {
            let file_name = format!("{}_{}.txt", name, i + 1);

            let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_name)
            .unwrap();

            files.push(file);
        }

        return files;
    }


    /// Calculcate steps from start nodes to Z node (simultaneously)
    /// 
    /// Actual steps to come up with the answer (not what is implemented)
    /// 1. Save nodes visited to file (one file per start node/moving entity)
    /// 2. Realize nodes visited follow a pattern that is reapeated every X_i * len(instructions) (i=0..len(nodes))
    /// 3. Realize that there is only one node ending with Z for each moving entity
    ///    *) This node is repeated with X_i * len(instructions) (i=0..len(nodes))
    /// 4. 1: JVZ steps=19783 (= 73 * 271)
    ///    2: MRZ steps=18157 (= 67 * 271)
    ///    3: TVZ steps=19241 (= 71 * 271)
    ///    4: KMZ steps=14363 (= 53 * 271)
    ///    5: ZZZ steps=15989 (= 59 * 271)
    ///    6: RCZ steps=12737 (= 47 * 271)
    /// 5. Find LCM of (73, 67, 71, 53, 47) = (as they are all prime numbers) 73 * 67 * 71 * 53 * 47 = 51 036 601 909
    /// 6. Steps = LCM(above) * 271 = 13 830 919 117 339
    pub fn steps_to_all_z(&self) -> u64 {
        let mut steps = 0;
        let mut max_z_nodes = 0;

        // Start nodes
        let mut nodes: Vec<&Node> = self.nodes.iter().filter(|(_, node)| node.label.ends_with('A')).map(|(_, node)| node).collect();

        let mut files = Self::open_appendable_files("output", nodes.len() as u32);

        loop {
            for c in self.instructions.chars() {
                nodes = nodes.iter().map(|node| {
                    if c == 'R' {
                        self.nodes.get(&node.right).unwrap()
                    } else if c == 'L' {
                        self.nodes.get(&node.left).unwrap()
                    } else {
                        panic!("Instruction not R or L!")
                    }
                }).collect();

                // TODO: remove (save to files)
                for (node, file) in nodes.iter().zip(files.iter_mut()) {
                    if let Err(e) = writeln!(file, "{}", node.label) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
                
                steps += 1;

                // Compare all nodes end label char with Z!
                let z_nodes = nodes.iter().filter(|node| node.label.ends_with('Z')).count();
                
                if z_nodes > max_z_nodes {
                    println!("{:?}", chrono::offset::Local::now());
                    println!("Seen {} matchin z nodes", z_nodes);
                    max_z_nodes = z_nodes;
                }

                // TODO: Revert
                // if z_nodes == nodes.len() {
                //     return steps;
                // }
                
                // TODO: Remove
                if steps == 500000 {
                    panic!()
                }
            }
        }

        return 0; // TODO: Remove
    }

    pub fn steps_to_node(&self, start_node: &str, end_node: &str) -> u64 {
        let mut node = self.nodes.get(start_node).unwrap();
        let mut steps = 0;
        
        loop {
            for c in self.instructions.chars() {
                node = 
                    if c == 'R' {
                        self.nodes.get(&node.right).unwrap()
                    } else if c == 'L' {
                        self.nodes.get(&node.left).unwrap()
                    } else {
                        panic!("Instruction not R or L!")
                    };

                steps += 1;

                if node.label == end_node {
                    return steps;
                }
            }
        }
    }

    pub fn parse(file: &str) -> Network {
        let lines = aoc_helper::read_lines(file);
        let mut it = lines.iter();

        let instructions = it.next().unwrap().to_string();

        it.next(); // Skip empty line

        let mut nodes = HashMap::new();
        let re = Regex::new(r"([^\s]+) = \(([^\s]+), ([^\s]+)\)").unwrap();
        while let Some(line) = it.next() {
            for (_, [node_label, node_left_label, node_right_label]) in re.captures_iter(line).map(|c| c.extract()) {
                let node = Node { label: node_label.to_string(), left: node_left_label.to_string(), right: node_right_label.to_string() };
                nodes.insert(node.label.to_string(), node);
                break;
            }
        }

        return Network { instructions: instructions, nodes: nodes }
    }
}