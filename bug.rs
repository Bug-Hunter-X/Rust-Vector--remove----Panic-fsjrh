fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before removing: {:?}", vec);

    // This will cause a panic if the vector is empty
    vec.remove(1);

    println!("After removing: {:?}", vec);
}