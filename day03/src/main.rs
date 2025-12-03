use std::io;

fn get_largest_ten(line: String) -> u64 {
    let max = line
        .chars()
        .rev()
        .skip(1)
        .map(|x| x.to_digit(10).unwrap() as u64)
        .enumerate()
        .max_by_key(|(_, x)| *x)
        .unwrap();

    let max2 = line
        .chars()
        .skip(line.len() - max.0 - 1)
        .map(|x| x.to_digit(10).unwrap() as u64)
        .enumerate()
        .max_by_key(|(_, x)| *x)
        .unwrap();

    print!(
        "{line} {} {} ({}) ({}) len: {} skipped: {}",
        max.1,
        max2.1,
        max.0,
        max2.0,
        line.len(),
        line.len() - max.0 - 1
    );
    max.1 * 10 + max2.1
}

fn main() {
    let mut count: u64 = 0;
    for line in io::stdin().lines() {
        match line {
            Ok(v) => count += get_largest_ten(v.clone()),
            Err(e) => break,
        }
        print!(" {count}\n");
    }
    println!("RESULT {count}");
}
