fn main() {
    let vec = vec![1, 2, 3];

    // Using a for loop to iterate
    for element in &vec {
        println!("Element: {}", element);
    }

    // Or clone the vec to use an iterator multiple times
    let vec_clone = vec.clone();
    let mut iter = vec_clone.iter();
    println!("First element: {}", iter.next().unwrap());
    println!("Second element: {}", iter.next().unwrap());
    println!("Third element: {}", iter.next().unwrap());
} 