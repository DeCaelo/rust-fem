fn main() {
    let items: Vec<_> = [1, 2, 3].iter().map(|f| f + 1).collect();

    println!("{:?}", items);
}
