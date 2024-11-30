fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let y = five();

    println!("The value of y is: {y}");

    println!("plus_one(7) is {}", plus_one(7));

}

fn five() -> i32 {
    return 5
}

fn another_function(x: u32, y: char) {
    println!("Hello, world from another function with passed in value {x}{y}!");
    //let x = y = 'a';
}

fn plus_one(x: u32) -> u32 {
    x + 1 // shouldn't put ; at the end of this line
}
