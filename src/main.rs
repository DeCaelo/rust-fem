fn main() {
    // how long do my variable live
    let data = vec![1, 2, 3];
    let mut items = data.iter().map(|f| f + 1);

    // collect()
    let mut collected_items = vec![];
    while let Some(x) = items.next() {
        collected_items.push(x);
    }

    println!("{:?}", collected_items);
}
