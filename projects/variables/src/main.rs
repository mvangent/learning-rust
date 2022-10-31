fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    println!("Three hours consist of {THREE_HOURS_IN_SECONDS} seconds");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 3;

    let y = y + 1;

    {
        let y = y * 2;
        println!("Inner scope y = {y}");
    }

    println!("Value of y = {y}")
}
