use std::collections::HashMap;
use crate::day20::modules::Module;
use crate::day20::pulses::{Pulse, PulseType};

pub struct ConjunctionModule {
    inputs: HashMap<String, PulseType>,
    outputs: Vec<String>,
}

impl ConjunctionModule {
    pub fn new(inputs: &[&str], outputs: &[&str]) -> ConjunctionModule {
        ConjunctionModule {
            inputs: inputs.iter().map(|i| (i.to_string(), PulseType::Low)).collect::<HashMap<String, PulseType>>(),
            outputs: outputs.iter().map(|i| i.to_string()).collect(),
        }
    }
}

impl Module for ConjunctionModule {
    fn receive(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.inputs.entry(pulse.source.to_string()).and_modify(|p| *p = pulse.pulse_type);

        if self.inputs.values().all(|p| *p == PulseType::High) {
            self.outputs.iter().map(|o| Pulse::new(&pulse.destination, o, PulseType::Low)).collect()
        } else {
            self.outputs.iter().map(|o| Pulse::new(&pulse.destination, o, PulseType::High)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_from_inputs() {
        let m = ConjunctionModule::new(&["a", "b"], &["c", "d"]);

        assert_eq!(m.inputs, HashMap::from([("a".to_string(), PulseType::Low), ("b".to_string(), PulseType::Low)]));
        assert_eq!(m.outputs, vec!["c".to_string(), "d".to_string()]);
    }

    #[test]
    fn updates_input_on_receive() {
        let mut m = ConjunctionModule::new(&["a", "b"], &["c", "d"]);

        let expected = vec![
            Pulse::new("broadcaster", "c", PulseType::High),
            Pulse::new("broadcaster", "d", PulseType::High),
        ];
        assert_eq!(m.receive(&Pulse::new("a", "broadcaster", PulseType::High)), expected);
        assert_eq!(m.inputs["a"], PulseType::High);
    }

    #[test]
    fn sends_low_pulse_when_all_input_are_high() {
        let mut m = ConjunctionModule::new(&["a", "b"], &["c", "d"]);

        let expected_a = vec![
            Pulse::new("broadcaster", "c", PulseType::High),
            Pulse::new("broadcaster", "d", PulseType::High),
        ];
        assert_eq!(m.receive(&Pulse::new("a", "broadcaster", PulseType::High)), expected_a);
        let expected_b = vec![
            Pulse::new("broadcaster", "c", PulseType::Low),
            Pulse::new("broadcaster", "d", PulseType::Low),
        ];
        assert_eq!(m.receive(&Pulse::new("b", "broadcaster", PulseType::High)), expected_b);
    }
}