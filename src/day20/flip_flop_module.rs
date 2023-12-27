use crate::day20::modules::Module;
use crate::day20::pulses::{Pulse, PulseType};

pub struct FlipFlopModule {
    on: bool,
    outputs: Vec<String>
}

impl FlipFlopModule {
    pub fn new(outputs: &[&str]) -> FlipFlopModule {
        FlipFlopModule { on: false, outputs: outputs.iter().map(|i| i.to_string()).collect() }
    }
}

impl Module for FlipFlopModule {
    fn receive(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match (&pulse.pulse_type, self.on) {
            (PulseType::High, _) => vec![],
            (PulseType::Low, true) => {
                self.on = false;
                self.outputs.iter().map(|output| Pulse::new(&pulse.destination, output, PulseType::Low)).collect()
            }
            (PulseType::Low, false) => {
                self.on = true;
                self.outputs.iter().map(|output| Pulse::new(&pulse.destination, output, PulseType::High)).collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::pulses::PulseType;
    use super::*;

    #[test]
    fn creates_new_at_off_by_default() {
        let m = FlipFlopModule::new(&["b"]);

        assert!(!m.on);
        assert_eq!(m.outputs, vec!["b"]);
    }

    #[test]
    fn outputs_nothing_on_high_signal() {
        let mut m = FlipFlopModule::new(&["b"]);

        assert_eq!(m.receive(&Pulse::new("_","_", PulseType::High)), vec![]);
        assert!(!m.on);
    }

    #[test]
    fn flips_on_low_signal() {
        let mut m = FlipFlopModule::new(&["b"]);

        assert_eq!(m.receive(&Pulse::new("_", "a", PulseType::Low)), vec![Pulse::new("a","b", PulseType::High)]);
        assert!(m.on);

        assert_eq!(m.receive(&Pulse::new("_", "a", PulseType::Low)), vec![Pulse::new("a","b", PulseType::Low)]);
        assert!(!m.on);
    }
}