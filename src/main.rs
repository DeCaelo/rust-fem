mod shapes;

use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use shapes::{
    circle::Circle,
    collisions::{Collidable, Contains, Points},
    rect::Rect,
};

enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").ok_or(anyhow::anyhow!("Invalid shape"))?;

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("Invalid shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Rect(rect) => return rect.points(),
            Shape::Circle(circ) => return circ.points(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Rect(rect) => return rect.contains_point(point),
            Shape::Circle(circ) => return circ.contains_point(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rect(rect) => return write!(f, "{}", rect),
            Shape::Circle(circ) => return write!(f, "{}", circ),
        }
    }
}

fn main() -> Result<()> {
    // read the file
    let file = std::fs::read_to_string("shapes")?;
    // convert each line into a shape (parse)
    let shapes = file
        .lines()
        .filter_map(|line| line.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    // json collision
    let collisions = shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .collect::<Vec<_>>();

    for (a, b) in collisions {
        println!("{} collides with {}", a, b);
    }

    return Ok(());
}
