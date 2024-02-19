#![allow(dead_code)]

use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Blue,       // 蓝色
    DarkBlue,   // 深蓝色
    Green,      // 绿色
    LightBlue,  // 浅蓝色
    LightGreen, // 浅绿色
    Orange,     // 橙色
    Pink,       // 粉红色
    Purple,     // 紫色
    Red,        // 红色
    Yellow,     // 黄色
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Blue => write!(f, "{}", "*".truecolor(0, 191, 255)),
            Color::DarkBlue => write!(f, "{}", "*".truecolor(0, 0, 255)),
            Color::Green => write!(f, "{}", "*".green()),
            Color::LightBlue => write!(f, "{}", "*".truecolor(0, 255, 255)),
            Color::LightGreen => write!(f, "{}", "*".truecolor(144, 238, 144)),
            Color::Orange => write!(f, "{}", "*".truecolor(255, 165, 0)),
            Color::Pink => write!(f, "{}", "*".truecolor(200, 20, 136)),
            Color::Purple => write!(f, "{}", "*".purple()),
            Color::Red => write!(f, "{}", "*".truecolor(255, 0, 0)),
            Color::Yellow => write!(f, "{}", "*".yellow()),
        }
        // let color = match self {
        //     Color::Blue => "B",
        //     Color::DarkBlue => "D",
        //     Color::Green => "G",
        //     Color::LightBlue => "b",
        //     Color::LightGreen => "g",
        //     Color::Orange => "O",
        //     Color::Pink => "P",
        //     Color::Purple => "p",
        //     Color::Red => "R",
        //     Color::Yellow => "Y",
        // };
        // write!(f, "{}", color.red())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    // * * * *
    // *
    Blue1,
    // * * * *
    //   *   *
    Blue2,

    // * * *
    //   *
    DarkBlue1,
    // * * *
    // *   *
    DarkBlue2,

    // * * *
    // * *
    Green1,
    // * * *
    //   *
    Green2,

    // * * * *
    //   *
    LightBlue1,
    // * * * *
    //   * *
    LightBlue2,

    // * * *
    // *
    LightGreen1,
    // * * *
    // *   *
    LightGreen2,

    // * * * *
    //   *
    Orange1,
    // * * * *
    // *   *
    Orange2,

    // * * * *
    //     *
    Pink1,
    // * * * *
    // * *
    Pink2,

    // * * *
    //   * *
    Purple1,
    // * * *
    //     *
    Purple2,

    // * * * *
    // *     *
    Red1,
    // * * * *
    // *
    Red2,

    // * * * *
    //     * *
    Yellow1,
    // * * * *
    //       *
    Yellow2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Angle {
    A0,
    A90,
    A180,
    A270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self {
        Pos { x, y }
    }
}

impl Shape {
    fn setted_pos(&self) -> Vec<Pos> {
        match &self {
            Shape::Blue1 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Blue2 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1), (1, 3)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::DarkBlue1 => vec![(0, 0), (0, 1), (0, 2), (1, 1)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::DarkBlue2 => vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Green1 => vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Green2 => vec![(0, 0), (0, 1), (0, 2), (1, 1)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::LightBlue1 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::LightBlue2 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::LightGreen1 => vec![(0, 0), (0, 1), (0, 2), (1, 0)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::LightGreen2 => vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Orange1 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Orange2 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Pink1 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Pink2 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Purple1 => vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Purple2 => vec![(0, 0), (0, 1), (0, 2), (1, 2)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Red1 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 3)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Red2 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Yellow1 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 2), (1, 3)]
                .into_iter()
                .map(Into::into)
                .collect(),
            Shape::Yellow2 => vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)]
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
    pub fn rotate_pos(&self, angle: Angle) -> Vec<Pos> {
        let pos = self.setted_pos();
        let rotate_pos = match angle {
            Angle::A0 => pos,
            Angle::A90 => pos.into_iter().map(|p| Pos { x: p.y, y: -p.x }).collect(),
            Angle::A180 => pos.into_iter().map(|p| Pos { x: -p.x, y: -p.y }).collect(),
            Angle::A270 => pos.into_iter().map(|p| Pos { x: -p.y, y: p.x }).collect(),
        };
        // move to left top - top
        let min_x = rotate_pos.iter().map(|p| p.x).min().unwrap();
        let mut formed_pos = if min_x < 0 {
            rotate_pos
                .into_iter()
                .map(|p| Pos {
                    x: p.x - min_x,
                    y: p.y,
                })
                .collect()
        } else {
            rotate_pos
        };
        formed_pos.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        let first_y = formed_pos[0].y;
        if first_y < 0 {
            formed_pos = formed_pos
                .into_iter()
                .map(|p| Pos {
                    x: p.x,
                    y: p.y - first_y,
                })
                .collect();
        }
        formed_pos
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub color: Color,
    pub possible_shapes: Vec<Shape>,
}
impl Block {
    pub fn new(color: Color, possible_shapes: Vec<Shape>) -> Self {
        Block {
            color,
            possible_shapes,
        }
    }
}

// pub fn full_blocks() -> Vec<Block> {
//     #[rustfmt::skip]
//     let result = vec![
//         Block::new(Color::LightGreen, vec![Shape::LightGreen2]),
//         Block::new(Color::Pink, vec![Shape::Pink1]),
//         Block::new(Color::LightBlue, vec![Shape::LightBlue2]),
//         Block::new(Color::Green, vec![Shape::Green1]),
//         Block::new(Color::DarkBlue, vec![Shape::DarkBlue1]),
//         Block::new(Color::Red, vec![Shape::Red2]),
//         Block::new(Color::Purple, vec![Shape::Purple2]),
//         Block::new(Color::Yellow, vec![Shape::Yellow2]),
//         Block::new(Color::Blue, vec![Shape::Blue2]),
//         Block::new(Color::Orange, vec![Shape::Orange1]),
//     ];
//     result
// }

pub fn full_blocks() -> Vec<Block> {
    #[rustfmt::skip]
    let result = vec![
        Block::new(Color::LightGreen, vec![Shape::LightGreen1, Shape::LightGreen2]),
        Block::new(Color::Pink, vec![Shape::Pink1, Shape::Pink2]),
        Block::new(Color::LightBlue, vec![Shape::LightBlue1, Shape::LightBlue2]),
        Block::new(Color::Green, vec![Shape::Green1, Shape::Green2]),
        Block::new(Color::DarkBlue, vec![Shape::DarkBlue1, Shape::DarkBlue2]),
        Block::new(Color::Red, vec![Shape::Red1, Shape::Red2]),
        Block::new(Color::Purple, vec![Shape::Purple1, Shape::Purple2]),
        Block::new(Color::Yellow, vec![Shape::Yellow1, Shape::Yellow2]),
        Block::new(Color::Blue, vec![Shape::Blue1, Shape::Blue2]),
        Block::new(Color::Orange, vec![Shape::Orange1, Shape::Orange2]),
    ];
    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_shape() {
        let shapes = vec![
            Shape::Blue1,
            Shape::Blue2,
            Shape::DarkBlue1,
            Shape::DarkBlue2,
            Shape::Green1,
            Shape::Green2,
            Shape::LightBlue1,
            Shape::LightBlue2,
            Shape::LightGreen1,
            Shape::LightGreen2,
            Shape::Orange1,
            Shape::Orange2,
            Shape::Pink1,
            Shape::Pink2,
            Shape::Purple1,
            Shape::Purple2,
            Shape::Red1,
            Shape::Red2,
            Shape::Yellow1,
            Shape::Yellow2,
        ];
        for shape in shapes {
            let pos = shape.rotate_pos(Angle::A0);
            println!("shape:{:?}, pos: {:?}", shape, pos);
            let pos = shape.rotate_pos(Angle::A90);
            println!("shape:{:?}, pos: {:?}", shape, pos);
            let pos = shape.rotate_pos(Angle::A180);
            println!("shape:{:?}, pos: {:?}", shape, pos);
            let pos = shape.rotate_pos(Angle::A270);
            println!("shape:{:?}, pos: {:?}", shape, pos);
        }
    }
}
