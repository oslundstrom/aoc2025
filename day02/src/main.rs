use std::io;
use std::io::Read;

fn get_divisor(n_digits: u32, subsize: u32) -> u64 {
    // sum((10**((m-subsize)*k/(nsubs-1)) for k in range(0,nsubs)))
    let mut sum: u64 = 0;
    let n_subs = n_digits / subsize;
    for k in 0..n_subs {
        sum += 10_u64.pow((n_digits - subsize) * k / (n_subs - 1));
    }
    println!("DIVISOR {sum} (m={n_digits}, i={n_subs})");

    sum
}

fn is_valid(num: u64) -> bool {
    let n_digits = num.ilog10() + 1;
    // let num_str = num.to_string();
    (1..(n_digits / 2 + 1)).any(|subsize| {
        if n_digits.is_multiple_of(subsize) {
            num.is_multiple_of(get_divisor(n_digits, subsize))
        } else {
            println!("{num} lenght {n_digits} is not multiple of {subsize}");
            false
        }
    })
}

fn main() {
    let mut stdin = io::stdin();
    let mut contents = String::new();
    stdin
        .read_to_string(&mut contents)
        .expect("Could not read from stdin");

    let mut count: u64 = 0;

    for entry in contents.split(",") {
        println!("{entry}");
        if let Some((start, end)) = entry.trim().split_once("-") {
            let start: u64 = start.parse().expect("Could not parse start");
            let end: u64 = end.parse().expect("Could not parse end");
            for num in start..=end {
                println!("number {num}");
                if is_valid(num) {
                    count += num;
                    println!("VALID {num}")
                }
            }
        } else {
            continue;
        }
    }
    println!("RESULT: {count}")
}
