fn main() {
    print_labeled_measurement(5, 'h');

    let x = five();
    let six = plus_one(x);

    println!("five plus one = {six}")

}

fn print_labeled_measurement(hours: i32, unit: char) {
    println!("The measurement is {hours} {unit}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
