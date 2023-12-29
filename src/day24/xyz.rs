use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn new(x: f64, y: f64, z: f64) -> Xyz {
        Xyz { x, y, z }
    }

    pub fn from(input: &str) -> Xyz {
        let (x, y, z) = input.split(", ").map(|v| v.parse::<f64>().unwrap()).collect_tuple().unwrap();
        Xyz { x, y, z }
    }
}