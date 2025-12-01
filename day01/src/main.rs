use std::io;


fn floor_div(a: i32, b: i32) -> i32 {
    let div = a / b;
    let rem = a % b;
    if rem != 0 && ((rem > 0) != (b > 0)) {
        div - 1
    } else {
        div
    }
}

fn rotate_dial(position: i32, instruction: String) -> (i32, i32) {
    let direction = &instruction[..1];
    let steps: i32 = instruction[1..].trim().parse().expect("Incorrect input!");
    println!("{direction} {steps}");
    let new_position: i32 = match direction {
        "R" => (position+steps) % 100,
        "L" => ((position-steps) % 100 + 100) % 100, // Euclidian modulo
        &_ => todo!(),
    };

    let mut count: i32 = match direction {
        "R" => (position+steps)/ 100,
        "L" => floor_div(steps-position, 100) + 1,
        &_ => todo!(),
    };

    if direction == "L" && position == 0 {
        count -= 1;
    }

    return (new_position, count);
}

fn main() {
    println!("Starting");
    let mut dial_position: i32 = 50;
    let mut static_counter: i32 = 0;
    let mut count: i32 = 0;
    let mut counter: i32 = 0;
    println!("Dial: {dial_position}");
    loop {
        let mut instruction = String::new();
        io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read line");

        if instruction.trim().is_empty() {
            println!("Final: {dial_position}");
            break;
        } else {
            (dial_position, count) = rotate_dial(dial_position, String::from(instruction));
        }

        if dial_position == 0 {
            static_counter += 1;
        }

        if count != 0 {
            counter += count;
            println!("Counter +{count} ({counter})");
        }
        count = 0;
        println!("Dial: {dial_position}");
        println!("");
    }
    println!("Static counter: {static_counter}");
    println!("Counter: {counter}");
}
