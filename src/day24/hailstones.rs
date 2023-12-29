use itertools::Itertools;
use z3::ast::{Ast, Int, Real};
use crate::day24::hailstone::Hailstone;
use crate::day24::xyz::Xyz;

pub struct Hailstones {
    hailstones: Vec<Hailstone>,
}

impl Hailstones {
    pub fn from(input: &str) -> Hailstones {
        Hailstones { hailstones: input.lines().map(Hailstone::from).collect() }
    }

    pub fn intersections_between(&self, min: f64, max: f64) -> usize {
        self.hailstones.iter()
            .tuple_combinations::<(_, _)>()
            .filter_map(|(a, b)| a.intersects(b))
            .filter(|(x, y)| (min..=max).contains(x) && (min..=max).contains(y))
            .count()
    }

    pub fn rock_position_to_obliterate_all_hailstone_to_dust(&self) -> Xyz {
        let ctx = z3::Context::new(&z3::Config::new());
        let solver = z3::Solver::new(&ctx);
        let [fx, fy, fz, fdx, fdy, fdz] = ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Real::new_const(&ctx, v));

        for (i, hailstone) in self.hailstones.iter().take(3).enumerate() {
            let [x, y, z, dx, dy, dz] = [hailstone.position.x, hailstone.position.y, hailstone.position.z, hailstone.velocity.x, hailstone.velocity.y, hailstone.velocity.z].map(|v| Int::from_i64(&ctx, v as i64).to_real());
            let time = Real::new_const(&ctx, format!("time{i}"));

            solver.assert(&((&x + &dx * &time)._eq(&(&fx + &fdx * &time))));
            solver.assert(&((&y + &dy * &time)._eq(&(&fy + &fdy * &time))));
            solver.assert(&((&z + &dz * &time)._eq(&(&fz + &fdz * &time))));
        }
        assert_eq!(solver.check(), z3::SatResult::Sat);

        let x = solver.get_model().unwrap().eval(&fx, true).unwrap().to_string().parse::<f64>().unwrap();
        let y = solver.get_model().unwrap().eval(&fy, true).unwrap().to_string().parse::<f64>().unwrap();
        let z = solver.get_model().unwrap().eval(&fz, true).unwrap().to_string().parse::<f64>().unwrap();

        Xyz::new(x, y, z)
    }
}