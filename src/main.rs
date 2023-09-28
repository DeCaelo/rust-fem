fn uncomplete_fn(x: usize) -> bool {
    if x < 10 {
        return true;
    }

    todo!("hey, finish me later")
}

fn only_evens(x: usize) -> bool {
    if x % 2 == 1 {
        unreachable!("this should never happen")
    }

    todo!("hey, finish me later")
}

fn main() {
    let foo = Some(5);

    let bar = foo.unwrap();
}
