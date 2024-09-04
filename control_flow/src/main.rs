fn main() {
    for_loop2();
}

fn for_loop2() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Liftoff!")
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {element}");
    }
}

fn while_loop() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("Liftoff!")
}

fn named_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1
    }

    println!("End count = {count}");
}
