use std::{fs::read_to_string, path::Path};

/*
 * Part 1
 *   A - rock - X - 1P
 *   B - paper - Y - 2P
 *   C - scissors - Z - 3P
 *
 *   Win = 6
 *   Draw = 3
 *   Lose = 0
 *
 * Part 2
 * X = lose
 * Y = draw
 * Z = win
 */

fn part1() {
    let mut score = 0;
    read_to_string(Path::new("input/input.txt"))
        .expect("Failed to read input")
        .as_str()
        .split("\n")
        .for_each(|entry| {
            match entry {
                "A X" => score += 3 + 1,
                "A Y" => score += 6 + 2,
                "A Z" => score += 0 + 3,
                "B X" => score += 0 + 1,
                "B Y" => score += 3 + 2,
                "B Z" => score += 6 + 3,
                "C X" => score += 6 + 1,
                "C Y" => score += 0 + 2,
                "C Z" => score += 3 + 3,
                _ => score += 0,
            };
        });

    println!("Part1: Strategy score: {score}");
}

fn part2() {
    let mut score = 0;
    read_to_string(Path::new("input/input.txt"))
        .expect("Failed to read input")
        .as_str()
        .split("\n")
        .for_each(|entry| {
            if let Some((opp, state)) = entry.split_once(' ') {
                match state.trim() {
                    "X" => match opp {
                        "A" => score += 3,
                        "B" => score += 1,
                        "C" => score += 2,
                        _ => score += 0,
                    },
                    "Y" => match opp {
                        "A" => score += 3 + 1,
                        "B" => score += 3 + 2,
                        "C" => score += 3 + 3,
                        _ => score += 0,
                    },
                    "Z" => match opp {
                        "A" => score += 6 + 2,
                        "B" => score += 6 + 3,
                        "C" => score += 6 + 1,
                        _ => score += 0,
                    },
                    _ => score += 0,
                }
            }
        });

    println!("Part2: Strategy score: {score}")
}

fn main() {
    part1();
    part2();
}
