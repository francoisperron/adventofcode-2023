use std::collections::HashMap;
use crate::day20::broadcaster_module::BroadcasterModule;
use crate::day20::conjunction_module::ConjunctionModule;
use crate::day20::flip_flop_module::FlipFlopModule;
use crate::day20::pulses::{Pulse, PulseType, Pulses};

pub trait Module {
    fn receive(&mut self, pulse: &Pulse) -> Vec<Pulse>;
}

pub struct Modules {
    modules: HashMap<String, Box<dyn Module>>
}

impl Modules {
    pub fn from(input: &str) -> Modules {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        for line in input.lines() {
            let (source, outputs) = line.split_once(" -> ").unwrap();
            let source = source.replace(['%', '&'], "");
            let outputs = outputs.split(", ").collect::<Vec<&str>>();

            let inputs: Vec<String> = input.lines()
                .map(|l| l.split_once(" -> ").unwrap())
                .filter(|(_, o)| o.contains(&source))
                .map(|(s, _)| s.replace(['%', '&'], ""))
                .collect();
            let inputs = inputs.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

            match line.chars().next().unwrap() {
                'b' => modules.insert(source, Box::new(BroadcasterModule::new(&outputs))),
                '%' => modules.insert(source, Box::new(FlipFlopModule::new(&outputs))),
                '&' => modules.insert(source, Box::new(ConjunctionModule::new(&inputs, &outputs))),
                _ => panic!()
            };
        }

        Modules { modules }
    }

    pub fn cycles_low_times_highs(&mut self, cycles: usize) -> usize {
        let mut total_highs = 0;
        let mut total_lows = 0;
        for _cycle in 0..cycles {
            let (highs, lows) = self.push_button();
            // println!("{:?} ({:?}, {:?})", _cycle, highs, lows);
            total_highs += highs;
            total_lows +=lows;
        }

        total_highs * total_lows
    }

    fn push_button(&mut self) -> (usize, usize) {
        let mut pulses = Pulses::new();
        pulses.push(Pulse::new("button", "broadcaster", PulseType::Low));

        while let Some(pulse) = pulses.pop() {
            // println!("\n{:?}", pulse);
            if let Some(module) = self.modules.get_mut(&pulse.destination) {
                let new_pulses = module.receive(&pulse);
                for new_pulse in new_pulses {
                    // println!("new {:?}", new_pulse);
                    pulses.push(new_pulse);
                }
            }
        }

        (pulses.highs, pulses.lows)
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::broadcaster_module::BroadcasterModule;
    use crate::day20::conjunction_module::ConjunctionModule;
    use crate::day20::flip_flop_module::FlipFlopModule;
    use super::*;

    #[test]
    fn button_low_to_broadcaster_low_to_a_high_to_inv_low_to_b() {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        modules.insert("broadcaster".to_string(), Box::new(BroadcasterModule::new(&["a"])));
        modules.insert("a".to_string(), Box::new(FlipFlopModule::new(&["inv"])));
        modules.insert("inv".to_string(), Box::new(ConjunctionModule::new(&["a"], &["b"])));

        let mut modules = Modules { modules };

        assert_eq!(modules.cycles_low_times_highs(1), 3);
    }

    #[test]
    fn example1_cycle1() {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        modules.insert("broadcaster".to_string(), Box::new(BroadcasterModule::new(&["a", "b", "c"])));
        modules.insert("a".to_string(), Box::new(FlipFlopModule::new(&["b"])));
        modules.insert("b".to_string(), Box::new(FlipFlopModule::new(&["c"])));
        modules.insert("c".to_string(), Box::new(FlipFlopModule::new(&["inv"])));
        modules.insert("inv".to_string(), Box::new(ConjunctionModule::new(&["c"], &["a"])));

        let mut modules = Modules { modules };

        assert_eq!(modules.cycles_low_times_highs(1), 8 * 4);
    }

    #[test]
    fn example1() {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        modules.insert("broadcaster".to_string(), Box::new(BroadcasterModule::new(&["a", "b", "c"])));
        modules.insert("a".to_string(), Box::new(FlipFlopModule::new(&["b"])));
        modules.insert("b".to_string(), Box::new(FlipFlopModule::new(&["c"])));
        modules.insert("c".to_string(), Box::new(FlipFlopModule::new(&["inv"])));
        modules.insert("inv".to_string(), Box::new(ConjunctionModule::new(&["c"], &["a"])));

        let mut modules = Modules { modules };

        assert_eq!(modules.cycles_low_times_highs(1000), 32000000);
    }

    #[test]
    fn example2() {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

        modules.insert("broadcaster".to_string(), Box::new(BroadcasterModule::new(&["a"])));
        modules.insert("a".to_string(), Box::new(FlipFlopModule::new(&["inv", "con"])));
        modules.insert("inv".to_string(), Box::new(ConjunctionModule::new(&["a"], &["b"])));
        modules.insert("b".to_string(), Box::new(FlipFlopModule::new(&["con"])));
        modules.insert("con".to_string(), Box::new(ConjunctionModule::new(&["a", "b"], &["output"])));

        let mut modules = Modules { modules };

        assert_eq!(modules.cycles_low_times_highs(1000), 11687500);
    }
}