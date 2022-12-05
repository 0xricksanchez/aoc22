use std::{fs::read_to_string, path::Path};

fn main() {
    let mut input: Vec<u64> = read_to_string(Path::new("input/input.txt"))
        .expect("Error opening file")
        .as_str()
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|snack| snack.parse::<u64>().unwrap_or(0))
                .sum()
        })
        .collect();
    input.sort();
    input.reverse();
    println!("Elf carrying the most snacks: {:?}", input[0]);
    println!(
        "Top 3 Snack carries hold: {} snacks in total!",
        input[0..=2].iter().sum::<u64>()
    );
}
