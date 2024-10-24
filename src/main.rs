fn main() {
    let x: i32; // declaration without initialization
    x = 5; // Initialization
    x = 20; // Error: cannot assign twice of immutable variable
    print!("{}",x)
}
