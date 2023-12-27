use std::collections::{HashMap, VecDeque};

// FlipFlop
// on/off
// high pulse -> nothing
// low pulse -> off -> on -> high pulse
// low pulse -> on -> off -> low pulse

// Conjunction
// Remembers pulse type for each sender connected to it (initially low)
// Pulse -> Update memory -> if all(memory) == high -> low pulse, otherwise -> high pulse

// Broadcast (single)
// Pulse in -> Pulse out (same)

// Button (single)
// Press -> low pulse to broadcaster

// Pulses are processed in the order they are sent

enum Pulse {
    Low,
    High
}

enum ModuleType {
    FlipFlop(bool), // Bool for on/off (true = on, false = off)
    Conjunction(HashMap<String, Pulse>), // Map node name to most recent type of input received (false = low, true = high)
    Broadcast,
    Button
}

struct ElectronicModule {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>
}

pub struct ElectronicMap {
    modules: HashMap<String, ElectronicModule>
}

impl ElectronicModule {
    fn receive_pulse(&mut self, pulse: Pulse, queue: &mut VecDeque<(String, Pulse)>) {
        // TODO: implement
        match self.module_type {
            ModuleType::FlipFlop(mut on) => {
                match pulse {
                    Pulse::High => return,
                    Pulse::Low => {
                        on = !on; // TODO: Does this update the type?

                        let out_pulse = if on { Pulse::High } else { Pulse::Low };
                        for destination in self.destinations.iter() {
                            queue.push_back((destination.to_string(), out_pulse));
                        }
                    }
                }
            },
            ModuleType::Conjunction(state) => {
                // TODO: We need sender name!
                
                // Update state

                // Determine output pulse level

                // Send pulse
            },
            ModuleType::Broadcast => {
                for destination in self.destinations.iter() {
                    queue.push_back((destination.to_string(), pulse));
                }
            },
            _ => panic!("receive_pulse called on unsupported module type!")
        }
    }

    fn find_sources(name: &str, lines: &Vec<String>) -> Vec<String> {
        let mut sources = Vec::new();

        for line in lines.iter() {
            let mut split_line = line.split(" -> ");
            let right_side = split_line.last().unwrap();
            right_side.split(", ").for_each(|dest| {
                if dest == name {
                    sources.push(name.to_string());
                }
            })
        }

        return sources;
    }

    fn parse(line: &str, lines: &Vec<String>) -> ElectronicModule {
        let mut split_line = line.split(" -> ");
        let left_side = split_line.next().unwrap();

        let name: String;
        let module_type: ModuleType;
        if left_side.contains('%') {
            name = left_side.split('%').last().unwrap().to_string();
            module_type = ModuleType::FlipFlop(false);
        } else if left_side.contains('&') {
            name = left_side.split('&').last().unwrap().to_string();
            let mut memory = HashMap::new();
            let sources = Self::find_sources(&name, lines);
            for source in sources { // Not sure why this cannot be done inside a for each closure...
                memory.insert(source, Pulse::Low);
            }
            module_type = ModuleType::Conjunction(memory);
        } else if left_side == "broadcaster" {
            name = String::from("broadcaster");
            module_type = ModuleType::Broadcast;
        } else {
            panic!("Unexpected left side: {}", left_side);
        }

        let right_side = split_line.next().unwrap();
        let destinations: Vec<String> = right_side.split(", ").map(|dest| dest.to_string()).collect();

        ElectronicModule { name: name, module_type: module_type, destinations: destinations }

    }
}

impl ElectronicMap {
    fn press_button(&self) -> (u64, u64) {
        let mut queue = VecDeque::new();
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        queue.push_back((String::from("broadcaster"), Pulse::Low));

        while let Some((receiver_str, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1
            }

            let receiver = self.modules.get(&receiver_str);
            if receiver.is_none() {
                continue;
            }

            // Receive pulse
            receiver.unwrap().receive_pulse(pulse, &queue);
        }

        return (low_pulses, high_pulses);
    }

    pub fn determine_pulses_sent(&self, button_presses: u64) -> (u64, u64) {
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // TODO: Should find out when the state is reset back to start state for faster performance?

        for _ in 0..button_presses {
            let res = self.press_button();
            low_pulses += res.0;
            high_pulses += res.1;
        }

        return (low_pulses, high_pulses);
    }

    pub fn parse(file: &str) -> ElectronicMap {
        let lines = aoc_helper::read_lines(file);

        let mut modules = HashMap::new();
        for line in lines.iter() {
            let module = ElectronicModule::parse(line, &lines);
            modules.insert(module.name.clone(), module);
        }

        ElectronicMap { modules: modules }
    }
}