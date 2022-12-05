use std::collections::HashSet;
use std::{fs::read_to_string, path::Path};

fn find_common(a: &str, b: &str) -> Option<char> {
    let mut cret: Option<char> = None;
    let (short, long) = if a.len() >= b.len() { (b, a) } else { (a, b) };
    let s: HashSet<char> = short.chars().collect();

    long.chars().for_each(|c| {
        if s.contains(&c) {
            cret = Some(c);
        }
    });
    cret
}

fn find_common3(a: &str, b: &str, c: &str) -> Option<char> {
    let mut cret: Option<char> = None;
    let b_hs: HashSet<char> = b.chars().collect();
    let c_hs: HashSet<char> = c.chars().collect();

    a.chars().for_each(|c| {
        if b_hs.contains(&c) && c_hs.contains(&c) {
            cret = Some(c);
        }
    });
    cret
}

fn common_val(c: Option<char>) -> usize {
    if let Some(chr) = c {
        if chr.is_lowercase() {
            return (chr as u8 - 96) as usize;
        } else {
            return (chr as u8 - 38) as usize;
        };
    }
    0
}

fn part1() {
    let mut sum: usize = 0;
    read_to_string(Path::new("input/input.txt"))
        .expect("Failed to read input")
        .as_str()
        .split('\n')
        .for_each(|line| {
            let sz = line.len();
            let (a, b) = line.split_at(sz / 2);
            sum += common_val(find_common(a, b));
        });
    println!("Priority sum: {sum}");
}

fn part2() {
    let mut sum: usize = 0;
    for chnk in read_to_string(Path::new("input/input.txt"))
        .expect("Failed to read input")
        .as_str()
        .split('\n')
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
    {
        sum += common_val(find_common3(chnk[0], chnk[1], chnk[2]));
    }

    println!("Priority sum item types: {sum}");
}

fn main() {
    part1();
    part2();
}
