fn main() {
    let mut v = vec![1, 2, 3];
    let value = v[0]; // Access the first element safely
    v[0] = 10; // Modify the vector safely
    println!("v: {:?}", v);
    println!("v[0]: {}", value);
}