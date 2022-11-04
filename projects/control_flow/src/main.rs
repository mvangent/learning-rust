fn main() {
    loop_magic();

    while_magic();

    for_in_magic();

    for_in_range_magic();

}

fn loop_magic() {
    let mut count = 0;

    'counting_up: loop {
        println!("Count = {count}");

        let mut remaining = 10;

        loop {
            println!("Remaining {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count: {count}");
}

fn while_magic() {
    let mut countdown = 3;

    while countdown != 0 {
        println!("{countdown}");

        countdown -= 1
    }

    println!("LIFT OFF!!")
}

fn for_in_magic() {
    let array = [10, 20, 30, 40, 50];

    for number in array {
        println!("{number}")
    }
}

fn for_in_range_magic() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("Lift off!! yeah")
}
