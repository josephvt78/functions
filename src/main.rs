fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
}

fn another_function(x: u32, y: char) {
    println!("Hello, world from another function with passed in value {x}{y}!");
}
