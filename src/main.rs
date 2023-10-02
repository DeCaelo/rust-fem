fn multiply(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    println!("value: {}", multiply(vec, 0));
}
