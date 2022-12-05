use std::{fs::read_to_string, path::Path};

fn main() {
    let mut sum_p1: usize = 0;
    let mut sum_p2: usize = 0;
    read_to_string(Path::new("input/input.txt"))
        .expect("Failed to read input")
        .as_str()
        .split('\n')
        .filter(|&x| !x.is_empty())
        .for_each(|line| {
            let elf_vec: Vec<u64> = line
                .splitn(4, [',', '-'])
                .map(|x| str::parse::<u64>(&x).unwrap())
                .collect::<Vec<_>>();
            let elf_a = (elf_vec[0], elf_vec[1]);
            let elf_b = (elf_vec[2], elf_vec[3]);
            if (elf_a.0 <= elf_b.0 && elf_b.1 <= elf_a.1)
                || (elf_b.0 <= elf_a.0 && elf_a.1 <= elf_b.1)
            {
                sum_p1 += 1;
            }
            if !(elf_a.1 < elf_b.0 || elf_b.1 < elf_a.0) {
                sum_p2 += 1
            }
        });
    println!("Complete overlaps: {sum_p1}");
    println!("Partial overlaps: {sum_p2}");
}
