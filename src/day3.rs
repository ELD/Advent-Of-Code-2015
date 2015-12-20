use std::collections::HashMap;
use std::collections::HashSet;

struct GridPointer {
    x: i32,
    y: i32,
}

// From a list of directions, how many houses receive at least one present?
pub fn get_solution_part1() -> i32 {
    let input = include_str!("day3input.txt");

    let mut location = GridPointer { x: 0, y: 0 };
    let mut visited_map: HashMap<(i32, i32), i32> = HashMap::new();
    visited_map.insert((location.x, location.y), 1);

    for dir in input.chars() {
        match dir {
            '^' => {
                location.y += 1;
                find_or_add_to_map(&mut visited_map, (location.x, location.y))
            }
            'v' => {
                location.y -= 1;
                find_or_add_to_map(&mut visited_map, (location.x, location.y))
            }
            '<' => {
                location.x -= 1;
                find_or_add_to_map(&mut visited_map, (location.x, location.y))
            }
            '>' => {
                location.x += 1;
                find_or_add_to_map(&mut visited_map, (location.x, location.y))
            }
            _ => {}
        }
    }

    visited_map.len() as i32
}

pub fn get_solution_part2() -> i32 {
    let input = include_str!("day3input.txt");

    let mut santa_location = GridPointer { x: 0, y: 0 };
    let mut robosanta_location = GridPointer { x: 0, y: 0 };
    let mut santa_visited_map: HashSet<(i32, i32)> = HashSet::new();
    let mut robosanta_visited_map: HashSet<(i32, i32)> = HashSet::new();
    let mut counter = 0;

    santa_visited_map.insert((santa_location.x, santa_location.y));
    santa_visited_map.insert((robosanta_location.x, robosanta_location.y));

    for dir in input.chars() {
        match dir {
            '^' => {
                if counter % 2 == 0 {
                    santa_location.y += 1;
                    santa_visited_map.insert((santa_location.x, santa_location.y));
                } else {
                    robosanta_location.y += 1;
                    robosanta_visited_map.insert((robosanta_location.x, robosanta_location.y));
                }
            }
            'v' => {
                if counter % 2 == 0 {
                    santa_location.y -= 1;
                    santa_visited_map.insert((santa_location.x, santa_location.y));
                } else {
                    robosanta_location.y -= 1;
                    robosanta_visited_map.insert((robosanta_location.x, robosanta_location.y));
                }
            }
            '<' => {
                if counter % 2 == 0 {
                    santa_location.x -= 1;
                    santa_visited_map.insert((santa_location.x, santa_location.y));
                } else {
                    robosanta_location.x -= 1;
                    robosanta_visited_map.insert((robosanta_location.x, robosanta_location.y));
                }
            }
            '>' => {
                if counter % 2 == 0 {
                    santa_location.x += 1;
                    santa_visited_map.insert((santa_location.x, santa_location.y));
                } else {
                    robosanta_location.x += 1;
                    robosanta_visited_map.insert((robosanta_location.x, robosanta_location.y));
                }
            }
            _ => {}
        }

        counter += 1;
    }

    (santa_visited_map.union(&robosanta_visited_map).collect::<Vec<&(i32, i32)>>()).len() as i32
}

fn find_or_add_to_map(map: &mut HashMap<(i32, i32), i32>, location: (i32, i32)) {
    let entry = map.entry(location).or_insert(1);
    *entry += 1;
}
