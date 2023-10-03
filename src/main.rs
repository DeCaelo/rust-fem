mod shapes;

use shapes::{circle::Circle, collisions::Collidable};

use crate::shapes::rect::Rect;

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();

    let circ = Circle {
        radius: 3.0,
        x: 1.0,
        y: 1.0,
    };

    let circ2 = Circle {
        radius: 2.0,
        x: 1.0,
        y: 1.0,
    };

    rect.collide(&rect2);
    circ.collide(&circ2);
    circ.collide(&rect);
}
