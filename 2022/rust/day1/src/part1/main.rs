use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut cur_elf = 0;
    let mut max_elf = -1;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            max_elf = if max_elf < cur_elf { cur_elf } else { max_elf };
            cur_elf = 0;

            continue;
        }

        let num: i32 = line.parse().unwrap();
        cur_elf += num
    }

    if cur_elf > max_elf {
        max_elf = cur_elf;
    }

    println!("{max_elf}");
}
