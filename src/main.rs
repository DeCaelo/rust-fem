mod shapes;

use crate::shapes::{area::Area, circle::Circle, rect::Rect};

fn main() {
    let rect = Rect::default();
    let circ = Circle::default();

    println!("{}", rect);
    println!("{}", circ);
}
