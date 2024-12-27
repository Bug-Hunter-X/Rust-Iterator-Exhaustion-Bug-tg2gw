fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());

    // Trying to access elements after the iterator has been exhausted
    println!("Third element (incorrect): {}", iter.next().unwrap());
}