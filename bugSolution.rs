fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Before removing: {:?}", vec);

    // Safe removal: Check bounds before removing
    if vec.len() > 1 {
        vec.remove(1);
    } else {
        println!("Vector too short to remove element at index 1");
    }

    println!("After removing: {:?}", vec);

    //Alternative using get() and unwrap_or()
    let mut vec2 = vec![1,2,3];
    let removed = vec2.get(1).and_then(|x| Some(*x));
    println!("Removed value: {:?}", removed);
    println!("vec2: {:?}", vec2);    
} 