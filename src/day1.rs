use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

fn read_input() -> String {
    let mut file = match File::open("src/day1input.txt") {
        Ok(file) => file,
        Err(what) => panic!("{}", Error::description(&what)),
    };

    let mut buffer: String = String::new();

    file.read_to_string(&mut buffer);

    buffer
}

pub fn get_solution_part1() -> i32 {
    let instructions = read_input();
    let mut floor = 0;

    for character in instructions.chars() {
        match character {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }
    }

    floor
}

pub fn get_solution_part2() -> i32 {
    let instructions = read_input();
    let mut floor = 0;
    let mut position = 0;

    for character in instructions.chars() {
        match character {
            '(' => {
                floor += 1;
                position += 1;
            },
            ')' => {
                floor -= 1;
                position += 1;
            },
            _ => {},
        }

        if floor < 0 {
            return position;
        }
    }

    position
}
