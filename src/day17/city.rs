use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::day17::block::Block;
use crate::day17::direction::{DIRECTIONS, START};
use crate::day17::position::Position;

pub struct Crucible {
    pub min_move: usize,
    pub max_move: usize,
}

#[derive(Debug)]
pub struct City {
    pub blocks: Vec<Vec<usize>>,
    pub end: Position,
}

impl City {
    pub fn from(input: &str) -> City {
        let blocks: Vec<Vec<usize>> = input.lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).map(|b| b as usize).collect())
            .collect();
        let end = Position(blocks.len() - 1, blocks[0].len() - 1);
        City { blocks, end }
    }

    // Dijkstra: https://doc.rust-lang.org/std/collections/binary_heap/index.html
    // Min-heap: https://doc.rust-lang.org/alloc/collections/binary_heap/struct.BinaryHeap.html#min-heap
    pub fn shortest_path(&self, crucible: &Crucible) -> usize {
        let mut visited = HashMap::new();
        let mut to_visit = BinaryHeap::new();

        to_visit.push(Reverse(Block::new(0, Position(0, 0), START)));

        while let Some(Reverse(block)) = to_visit.pop() {
            if block.position == self.end {
                return block.cost;
            }

            if visited.get(&block.hash()).is_some_and(|&visited_cost| block.cost > visited_cost) {
                continue;
            }

            for direction in DIRECTIONS.into_iter().filter(|direction| direction.on_other_axis(&block.direction)) {
                let mut next_cost = block.cost;

                for next_move in 1..=crucible.max_move {
                    let new_position = block.position.move_to(&direction, next_move);
                    if self.is_outside(new_position) {
                        continue;
                    }

                    next_cost += self.heat_at(new_position);

                    if next_move < crucible.min_move {
                        continue;
                    }

                    let next_block = Block::new(next_cost, new_position, direction);

                    if next_cost < *visited.get(&next_block.hash()).unwrap_or(&usize::MAX) {
                        visited.insert(next_block.hash(), next_cost);
                        to_visit.push(Reverse(next_block));
                    }
                }
            }
        }
        panic!("No path to end")
    }

    pub fn is_outside(&self, position: Position) -> bool {
        position.0 >= self.blocks.len() || position.1 >= self.blocks[0].len()
    }

    pub fn heat_at(&self, position: Position) -> usize {
        self.blocks[position.0][position.1]
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::city::City;
    use crate::day17::position::Position;

    #[test]
    fn parses_city() {
        let city = City::from("123\n456");

        assert_eq!(city.blocks, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(city.end, Position(1, 2));
    }

    #[test]
    fn checks_if_position_is_outside() {
        let city = City::from("123\n456");

        assert!(city.is_outside(Position(1, 3)));
        assert!(city.is_outside(Position(2, 2)));
    }

    #[test]
    fn gets_heat_at_position() {
        let city = City::from("123\n456");

        assert_eq!(city.heat_at(Position(0,0)), 1);
        assert_eq!(city.heat_at(Position(1,2)), 6);
    }
}