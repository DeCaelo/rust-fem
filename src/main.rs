enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    // &self is a readonly fct
    fn is_green(&self) -> bool {
        // pattern matching
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}

fn main() {
    print_color(Color::Red);
    Color::Green.is_green();
}
