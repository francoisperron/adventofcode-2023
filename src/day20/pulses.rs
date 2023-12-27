use std::collections::VecDeque;

pub struct Pulses {
    pulses: VecDeque<Pulse>,
    pub highs: usize,
    pub lows: usize,
    pub history: Vec<Pulse>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pulse {
    pub pulse_type: PulseType,
    pub source: String,
    pub destination: String,
}

impl Pulse {
    pub fn new(source: &str, destination: &str, pulse_type: PulseType) -> Pulse {
        Pulse { source: source.to_string(), destination: destination.to_string(), pulse_type }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PulseType {
    Low,
    High,
}

impl Pulses {
    pub fn new() -> Pulses {
        Pulses { pulses: VecDeque::new(), highs: 0, lows: 0, history: vec![] }
    }

    pub fn push(&mut self, pulse: Pulse) {
        self.count(&pulse.pulse_type);
        self.pulses.push_back(pulse.clone());
        self.history.push(pulse);
    }

    pub fn pop(&mut self) -> Option<Pulse> {
        self.pulses.pop_front()
    }

    fn count(&mut self, pulse_type: &PulseType) {
        match pulse_type {
            PulseType::High => self.highs += 1,
            PulseType::Low => self.lows += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_highs_and_lows() {
        let mut pulses = Pulses::new();

        pulses.push(Pulse::new("a", "b", PulseType::High));
        pulses.push(Pulse::new("a", "b", PulseType::High));
        pulses.push(Pulse::new("a", "b", PulseType::Low));

        assert_eq!(pulses.highs, 2);
        assert_eq!(pulses.lows, 1);
    }
}