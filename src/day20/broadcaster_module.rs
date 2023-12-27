use crate::day20::modules::Module;
use crate::day20::pulses::Pulse;

pub struct BroadcasterModule {
    outputs: Vec<String>,
}

impl BroadcasterModule {
    pub fn new(outputs: &[&str]) -> BroadcasterModule {
        BroadcasterModule { outputs: outputs.iter().map(|o| o.to_string()).collect() }
    }
}

impl Module for BroadcasterModule {
    fn receive(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.outputs.iter().map(|o| Pulse::new(&pulse.destination, o, pulse.pulse_type)).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::pulses::PulseType;
    use super::*;

    #[test]
    fn broadcasts_pulse_to_all_outputs() {
        let mut m = BroadcasterModule::new(&["a", "b"]);

        let expected = vec![
            Pulse::new("broadcaster", "a", PulseType::Low),
            Pulse::new("broadcaster", "b", PulseType::Low)
        ];
        assert_eq!(m.receive(&Pulse::new("_", "broadcaster", PulseType::Low)), expected);
    }
}