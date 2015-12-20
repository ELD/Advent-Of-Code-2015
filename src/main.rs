extern crate advent_of_code;

use advent_of_code::{day1, day2, day3, day4, day5, day6};

fn main() {
    println!("Day 1 part 1 solution: {}", day1::get_solution_part1());
    println!("Day 1 part 2 solution: {}", day1::get_solution_part2());
    println!("------------");
    println!("Day 2 part 1 solution: {}", day2::part1_solution());
    println!("Day 2 part 2 solution: {}", day2::part2_solution());
    println!("------------");
    println!("Day 3 part 1 solution: {}", day3::get_solution_part1());
    println!("Day 3 part 2 solution: {}", day3::get_solution_part2());
    if cfg!(features = "day4") {
        println!("------------");
        println!("Day 4 part 1 solution: {}", day4::get_solution_part1());
        println!("Day 4 part 2 solution: {}", day4::get_solution_part2());
    } else {
        println!("Skipping day 4...");
    }
    println!("------------");
    println!("Day 5 part 1 solution: {}", day5::get_solution_part1());
    println!("Day 5 part 2 solution: {}", day5::get_solution_part2());
    println!("------------");
    println!("Day 6 part 1 solution: {}", day6::get_solution_part1());
    println!("Day 6 part 2 solution: {}", day6::get_solution_part2());
}
