use regex::Regex;
use std::io;
use std::io::Read;

fn main() {
    let re = Regex::new(r"^(\d+)+$").unwrap();

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
                let n_digits = num.ilog10() + 1;
                let num_str = num.to_string();
                if let Some(caps) = re.captures(&num_str) {
                    count += num;
                    println!("MATCH {num} [tally: {count}] {}", &caps[0]);
                } else {
                    println!("Odd {num} {n_digits}")
                }
            }
        } else {
            continue;
        }
    }
    println!("RESULT: {count}")
}
