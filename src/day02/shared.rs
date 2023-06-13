#[derive(Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors
}

impl From<Shape> for u32 {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 0,
            Shape::Paper => 1,
            Shape::Scissors => 2,
        }
    }
}

pub enum Outcome {
    Win(Shape),
    Draw(Shape),
    Lose(Shape),
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win(t) => 6 + u32::from(*t),
            Outcome::Draw(t) => 3 + u32::from(*t),
            Outcome::Lose(t) => u32::from(*t),
        }
    }
}

