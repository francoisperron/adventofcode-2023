use std::collections::HashMap;
use crate::day22::brick::Brick;

pub struct Bricks {
    bricks: Vec<Brick>,
}

impl Bricks {
    pub fn from(input: &str) -> Bricks {
        Bricks { bricks: input.lines().map(Brick::from).collect() }
    }

    pub fn safe_bricks_count(&mut self) -> usize {
        let (bricks_support, _) = self.fall_until_settled();

        self.bricks.iter()
            .filter(|b| !bricks_support.values().any(|support| support.len() == 1 && support.contains(b)))
            .count()
    }

    pub fn disintegrate_bricks_count(&mut self) -> usize {
        self.fall_until_settled();
        let original_bricks = self.bricks.clone();

        (0..self.bricks.len())
            .map(|i| {
                self.bricks = original_bricks.clone();
                self.bricks.remove(i);
                self.fall_until_settled().1
            })
            .sum()
    }

    pub fn fall_until_settled(&mut self) -> (HashMap<Brick, Vec<Brick>>, usize) {
        self.bricks.sort_by_key(|&Brick((_, _, z), _)| z);

        let mut bricks_support: HashMap<Brick, Vec<Brick>> = HashMap::new();
        let mut stopped_bricks: HashMap<usize, Vec<Brick>> = HashMap::new();
        let mut fallen_bricks = 0;

        for brick in self.bricks.iter_mut() {
            let mut fallen = false;
            while brick.is_in_the_air() {
                let mut falling = true;

                for next_brick in stopped_bricks.get(&(brick.0.2 - 1)).unwrap_or(&vec![]) {
                    if brick.encounters(next_brick) {
                        bricks_support.entry(*brick).or_default().push(*next_brick);
                        falling = false;
                    }
                }

                if falling {
                    brick.fall();
                    if !fallen {
                        fallen_bricks += 1;
                        fallen = true;
                    }
                } else {
                    break;
                }
            }
            stopped_bricks.entry(brick.1.2).or_default().push(*brick);
        }
        (bricks_support, fallen_bricks)
    }
}

