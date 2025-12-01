use std::io;

fn rotate_dial(position: i32, instruction: String) -> i32 {
    let direction = &instruction[..1];
    let steps: i32 = instruction[1..].trim().parse().expect("Incorrect input!");
    let new_position: i32 = match direction {
        "R" => (position+steps) % 100,
        "L" => ((position-steps) % 100 + 100) % 100, // Euclidian modulo
        &_ => todo!(),
    };

    return new_position;
}

fn main() {
    println!("Hello, world!");
    let mut dial_position: i32 = 50;
    loop {
        let mut instruction = String::new();
        io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read line");

        if instruction.trim().is_empty() {
            println!("Final: {dial_position}");
            break;
        } else {
            println!("{instruction}");
            dial_position = rotate_dial(dial_position, String::from(instruction));
            println!("{dial_position}");
        }
    }
}
