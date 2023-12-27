use std::collections::{HashMap, VecDeque, HashSet};

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

#[derive(Clone, Debug, PartialEq)]
enum Pulse {
    Low,
    High
}

#[derive(Clone)]
enum ModuleType {
    FlipFlop(bool), // Bool for on/off (true = on, false = off)
    Conjunction(HashMap<String, Pulse>), // Map node name to most recent type of input received (false = low, true = high)
    Broadcast,
    //Button
}

#[derive(Debug)]
struct Observe {
    name: String,
    pulse: Pulse,
    observations: Vec<u64>
}

#[derive(Clone)]
struct ElectronicModule {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>
}

pub struct ElectronicMap {
    modules: HashMap<String, ElectronicModule>
}

impl ElectronicModule {
    fn receive_pulse(&mut self, sender_str: String, pulse: Pulse, queue: &mut VecDeque<(String, String, Pulse)>) {
        match &mut self.module_type {
            ModuleType::FlipFlop(on) => {
                match pulse {
                    Pulse::High => return,
                    Pulse::Low => {
                        *on = !*on;

                        let out_pulse = if *on { Pulse::High } else { Pulse::Low };
                        for destination in self.destinations.iter() {
                            queue.push_back((destination.to_string(), self.name.clone(), out_pulse.clone()));
                        }
                    }
                }
            },
            ModuleType::Conjunction(state) => {
                // Update memory/state
                let single_state = state.get_mut(&sender_str).unwrap();
                *single_state = pulse;

                // Determine output pulse level
                let all_high = state.iter().map(|(_, last_pulse)| {
                    match last_pulse {
                        Pulse::High => true,
                        Pulse::Low => false
                    }
                }).all(|b| b);

                let out_pulse = if all_high { Pulse::Low } else { Pulse::High};

                // Send pulse
                for destination in self.destinations.iter() {
                    queue.push_back((destination.to_string(), self.name.clone(), out_pulse.clone()));
                }
            },
            ModuleType::Broadcast => {
                for destination in self.destinations.iter() {
                    queue.push_back((destination.to_string(), self.name.clone(), pulse.clone()));
                }
            },
        }
    }

    fn find_sources(name: &str, lines: &Vec<String>) -> Vec<String> {
        let mut sources = Vec::new();

        for line in lines.iter() {
            let mut split_line: std::str::Split<'_, &str> = line.split(" -> ");
            let left_side = split_line.next().unwrap();
            let right_side = split_line.last().unwrap();
            right_side.split(", ").for_each(|dest| {
                if dest == name {
                    let source = left_side.replace("%", "").replace("&", "");
                    sources.push(source);
                }
            });
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
            sources.into_iter().for_each(|source| { memory.insert(source, Pulse::Low); } );
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
    fn create_sub_map(&self, end_node_str: &str) -> ElectronicMap {
        let mut module_names = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(end_node_str.to_owned());

        // Determine nodes that are needed
        while let Some(name) = queue.pop_front() {
            for (mod_name, module) in self.modules.iter() {
                if module.destinations.contains(&name) && !module_names.contains(mod_name) {
                    queue.push_back(mod_name.to_owned());
                }
            }

            module_names.insert(name);
        }

        // Add needed nodes
        let mut modules = HashMap::new();
        module_names.into_iter().for_each(|name| {
            let module = self.modules.get(&name).unwrap().clone();
            modules.insert(name, module);
        });

        ElectronicMap { modules: modules }
    }

    fn press_button(&mut self, press_i: u64, mut observe: Option<&mut Observe>) -> (u64, u64) {
        let mut queue = VecDeque::new();
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // Simulates button press
        queue.push_back((String::from("broadcaster"), String::from("button"), Pulse::Low));

        while let Some((receiver_str, sender_str, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1
            }

            let receiver = self.modules.get_mut(&receiver_str);
            if receiver.is_none() { // E.g. for virtual "output" node
                if observe.is_some() {
                    let observe_unwrapped = observe.unwrap();

                    if sender_str == observe_unwrapped.name && pulse == observe_unwrapped.pulse {
                        observe_unwrapped.observations.push(press_i);
                    }

                    observe = Some(observe_unwrapped); // Ugly!
                }
                continue;
            }

            // Receive pulse
            receiver.unwrap().receive_pulse(sender_str, pulse, &mut queue);
        }

        return (low_pulses, high_pulses);
    }

    fn observe(&mut self, module_name: &str, pulse: Pulse) {
        let mut observe = Observe { name: module_name.to_owned(), pulse: pulse, observations: Vec::new() };
        let button_presses = 20000;

        for i in 1..(button_presses + 1) {
            let _ = self.press_button(i, Some(&mut observe));
        }

        println!("Observed: {:?}", observe);
        let diffs: Vec<u64> = observe.observations.iter().enumerate().map(|(i, val)| {
            if i == 0 {
                *val
            } else {
                *val - observe.observations[i - 1]
            }
        }).collect();
        println!("Diffs: {:?}", diffs);
    }

    pub fn buttons_press_required_for_low_rx(&mut self) -> u64 {
        // In input, only "&zh -> rx" can send to rx
        // &zh depends on &vd, &ns, &bh, &dl (which in turn depend on four large &??)
        
        // TODO: Find nodes dependent on zh (manual for now :D)
        let zh_senders = vec!["vd", "ns", "bh", "dl"];

        // Split these nodes into sub-electronic maps
        let mut sub_maps: Vec<ElectronicMap> = zh_senders.iter().map(|sender| self.create_sub_map(sender)).collect();

        // Observe when all these nodes send "high pulses" and find a common index
        sub_maps.iter_mut().enumerate().for_each(|(i, map)| map.observe(zh_senders[i], Pulse::High));

        // In the observations printed in map.observe, we can see that there is a pattern for when each of the four nodes sends "high pulse"
        // They all occur every x_i button press, where all x_i are prime numbers, so just multiply them together to get the answer
        // Did this part manually :D

        return 0; // TODO: Change
    }

    pub fn determine_pulses_sent(&mut self, button_presses: u64) -> (u64, u64) {
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // TODO: Should find out when the state is reset back to start state for faster performance?

        for i in 0..button_presses {
            let res = self.press_button(i, None);
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