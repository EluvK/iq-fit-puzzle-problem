use blocks::{Angle, Block, Color};
use itertools::Itertools;

mod blocks;

struct Map {
    grid: [[Option<Color>; COL as usize]; ROW as usize],
    is_occupied: [[bool; COL as usize]; ROW as usize],
}

const ROW: i32 = 5;
const COL: i32 = 10;

fn print_map(map: &Map) {
    for row in map.grid.iter() {
        for cell in row.iter() {
            if let Some(color) = cell {
                print!("|{}", color);
            } else {
                print!("| ");
            }
        }
        println!();
    }
}

impl Map {
    fn new() -> Map {
        Map {
            grid: [[None; COL as usize]; ROW as usize],
            is_occupied: [[false; COL as usize]; ROW as usize],
        }
    }
    fn set_occupied(&mut self, pos: Vec<(i32, i32)>, color: Color) {
        for p in pos {
            self.is_occupied[p.0 as usize][p.1 as usize] = true;
            self.grid[p.0 as usize][p.1 as usize] = Some(color);
        }
    }
    fn is_occupied(&self, pos: (i32, i32)) -> bool {
        self.is_occupied[pos.0 as usize][pos.1 as usize]
    }
    fn check(&self) -> bool {
        for row in self.is_occupied.iter() {
            for cell in row.iter() {
                if !cell {
                    return false;
                }
            }
        }
        true
    }

    fn dfs(&mut self, pos: (i32, i32), possible_blocks: Vec<Block>) {
        // println!("begin dfs {:?} with size {}", pos, possible_blocks.len());
        if possible_blocks.is_empty() && self.check() {
            println!("-------");
            print_map(self);
            return;
        }
        let dfs_block = possible_blocks.clone();
        for block in possible_blocks.clone() {
            let possible_shapes = block.possible_shapes.clone();
            let possible_angles = vec![Angle::A0, Angle::A90, Angle::A180, Angle::A270];
            for (shape, angle) in possible_shapes
                .into_iter()
                .cartesian_product(possible_angles)
            {
                
                let rotated_pos = shape.rotate_pos(angle);
                let mut can_set = true;
                for p in rotated_pos.clone() {
                    let new_pos = (pos.0 + p.x, pos.1 + p.y);
                    if new_pos.0 < 0 || new_pos.0 >= ROW || new_pos.1 < 0 || new_pos.1 >= COL {
                        can_set = false;
                        break;
                    }
                    if self.is_occupied(new_pos) {
                        can_set = false;
                        break;
                    }
                }
                if can_set {
                    for p in rotated_pos.clone() {
                        let new_pos = (pos.0 + p.x, pos.1 + p.y);
                        // println!("set {:?} as occupied", new_pos);
                        self.is_occupied[new_pos.0 as usize][new_pos.1 as usize] = true;
                        self.grid[new_pos.0 as usize][new_pos.1 as usize] =
                            Some(block.color.clone());
                    }
                    let mut next_blocks = dfs_block.clone();
                    next_blocks.retain(|b| *b != block);
                    let mut next_pos = (0, 0);
                    if !next_blocks.is_empty() {
                        while self.is_occupied(next_pos) {
                            next_pos.1 += 1;
                            if next_pos.1 == COL {
                                next_pos.1 = 0;
                                next_pos.0 += 1;
                            }
                            if next_pos.0 == ROW {
                                break;
                            }
                        }
                    }
                    if next_pos.0 != ROW {
                        self.dfs(next_pos, next_blocks);
                    }

                    for p in rotated_pos {
                        let new_pos = (pos.0 + p.x, pos.1 + p.y);
                        self.is_occupied[new_pos.0 as usize][new_pos.1 as usize] = false;
                        self.grid[new_pos.0 as usize][new_pos.1 as usize] = None;
                    }
                }
                // }
            }
        }
    }
}

fn main() {
    let mut blocks = blocks::full_blocks();
    let mut map = Map::new();
    // map.set_occupied(vec![(2, 0), (1, 1), (2, 1), (3, 1)], Color::DarkBlue);
    // map.set_occupied(vec![(2, 2), (0, 3), (1, 3), (2, 3), (3, 3)], Color::Pink);
    // map.set_occupied(vec![(1, 4), (2, 4), (3, 4), (4, 4), (4, 5)], Color::Red);
    // blocks
    //     .retain(|b| b.color != Color::Red && b.color != Color::Pink && b.color != Color::DarkBlue);

    // 120
    // map.set_occupied(vec![(0, 3), (1, 2), (1, 3), (1, 4)], Color::Green);
    // map.set_occupied(
    //     vec![(3, 5), (3, 6), (4, 4), (4, 5), (4, 6), (4, 7)],
    //     Color::LightBlue,
    // );
    // blocks.retain(|b| b.color != Color::Green && b.color != Color::LightBlue);

    // 118
    map.set_occupied(
        vec![(3, 0), (4, 0), (3, 2), (4, 1), (4, 2), (4, 3)],
        Color::Blue,
    );
    map.set_occupied(vec![(3, 4), (3, 6), (4, 4), (4, 5), (4, 6)], Color::Purple);
    blocks.retain(|b| b.color != Color::Blue && b.color != Color::Purple);

    map.dfs((0, 0), blocks);

    // println!("{:?}", map.grid);
}
