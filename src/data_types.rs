pub fn data_types() {
    let a: i32 = 10; // Integer
    let b: f64 = 3.14; // Floating point
    let c: char = 'A'; // Character
    let d: bool = true; // Boolean
    let e: &str = "Hello, Rust!"; // String slice
    let f: String = String::from("Hello, Rust!"); // Owned string
    let g: [i32; 3] = [1, 2, 3]; // Fixed-size array
    let h: Vec<i32> = vec![1, 2, 3]; // Dynamic array (vector)
    let i: (i32, f64, char) = (42, 2.718, 'Z'); // Tuple
    let j: Option<i32> = Some(5); // Optional value
    let k: Result<i32, &str> = Ok(10); // Result type
    let l: Box<i32> = Box::new(20); // Heap-allocated integer

    println!("Integer: {}, Float: {}, Char: {}, Bool: {}, String slice: {}, Owned string: {}, Fixed-size array: {:?}, Vector: {:?}, Tuple: {:?}, Optional value: {:?}, Result type: {:?}, Boxed integer: {}", 
             a, b, c, d, e, f, g, h, i, j, k, l);
}