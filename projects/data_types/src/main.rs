use std::io;

fn main() {
    // SCALAR TYPES
    // Integers
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // Floating point numbers
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.1;
    let product = 4 * 40;
    let quotient = 2.0 / 3.0;
    let floored = 2 / 3; // Results in 0
    let remainder = 5 % 3;

    // Boolean types
    let t = true;

    let f: bool = false;

    // Character type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';


    // COMPOUND TYPES
    // Tuple type
    let tup: (i32, f64, u8) = (-500, 3.14159265359, 1);
    
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let minus_five_hundred = tup.0;
    let pi = tup.1;
    let one = tup.2;

    // Array Type (only useful for fixed lengths, otherwise Vector is the type to go for)
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // same as let b = [3, 3, 3, 3, 3];
    
    let first = a[0];
    let second = a[1];

    println!("Please enter array index");

    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element = a[index];

    println!("The value at index {index} is {element}");
}
