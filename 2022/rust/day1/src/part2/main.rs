use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut cur_elf = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(cur_elf);
            cur_elf = 0;

            continue;
        }

        cur_elf += line.parse::<u32>().unwrap()
    }

    if cur_elf > 0 {
        elves.push(cur_elf);
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!("{}", elves[0] + elves[1] + elves[2]);
}
