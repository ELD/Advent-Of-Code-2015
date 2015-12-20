pub fn get_solution_part1() -> i32 {
    let input = include_str!("day6input.txt");

    let mut light_grid: [[bool;1000];1000] = [[false;1000];1000];

    for line in input.lines() {
        let tokenized_input: Vec<_> = line.split_whitespace().collect();
        // Toggle
        if tokenized_input.len() == 4 {
            // split token 1 and 3
            let beginning_coords: Vec<_> = tokenized_input[1].split(",").collect();
            let end_coords: Vec<_> = tokenized_input[3].split(",").collect();
            for i in beginning_coords[0].parse::<usize>().ok().expect("error")..end_coords[0].parse::<usize>().ok().expect("error") + 1 {
                for j in beginning_coords[1].parse::<usize>().ok().expect("error")..end_coords[1].parse::<usize>().ok().expect("error") + 1 {
                    light_grid[i][j] = !light_grid[i][j];
                }
            }
        }
        // Turn on/off
        else {
            let beginning_coords: Vec<_> = tokenized_input[2].split(",").collect();
            let end_coords: Vec<_> = tokenized_input[4].split(",").collect();
            for i in beginning_coords[0].parse::<usize>().ok().expect("error")..end_coords[0].parse::<usize>().ok().expect("error") + 1 {
                for j in beginning_coords[1].parse::<usize>().ok().expect("error")..end_coords[1].parse::<usize>().ok().expect("error") + 1 {
                    if tokenized_input[1] == "on" {
                        light_grid[i][j] = true;
                    } else {
                        light_grid[i][j] = false;
                    }
                }
            }
        }
    }

    let mut count = 0;
    for i in 1..1000 {
        for j in 1..1000 {
            if light_grid[i][j] {
                count += 1;
            }
        }
    }

    count
}

#[derive(Copy, Clone)]
struct GridCell {
    brightness: i32
}

impl GridCell {
    fn new() -> Self {
        GridCell { brightness: 0 }
    }

    fn toggle(&mut self) {

        self.brightness += 2;
    }

    fn turn_on(&mut self) {
        self.brightness += 1;
    }

    fn turn_off(&mut self) {
        if self.brightness < 1 {
            return;
        }

        self.brightness -= 1;
    }
}

pub fn get_solution_part2() -> i32 {
    let input = include_str!("day6input.txt");

    let mut grid: [[GridCell; 1000];1000] = [[GridCell::new(); 1000];1000];

    for line in input.lines() {
        let tokenized_line: Vec<_> = line.split_whitespace().collect();

        if tokenized_line.len() == 4 {
            let beginning_coords: Vec<_> = tokenized_line[1].split(",").collect();
            let end_coords: Vec<_> = tokenized_line[3].split(",").collect();
            let start_x: usize = beginning_coords[0].parse().ok().expect("error");
            let start_y: usize = beginning_coords[1].parse().ok().expect("error");
            let end_x: usize = end_coords[0].parse().ok().expect("error");
            let end_y: usize = end_coords[1].parse().ok().expect("error");

            for i in start_x..end_x + 1 {
                for j in start_y..end_y + 1 {
                    grid[i][j].toggle();
                }
            }
        } else {
            if tokenized_line[1] == "on" {
                let beginning_coords: Vec<_> = tokenized_line[2].split(",").collect();
                let end_coords: Vec<_> = tokenized_line[4].split(",").collect();
                let start_x: usize = beginning_coords[0].parse().ok().expect("error");
                let start_y: usize = beginning_coords[1].parse().ok().expect("error");
                let end_x: usize = end_coords[0].parse().ok().expect("error");
                let end_y: usize = end_coords[1].parse().ok().expect("error");

                for i in start_x..end_x + 1 {
                    for j in start_y..end_y + 1 {
                        grid[i][j].turn_on();
                    }
                }
            } else {
                let beginning_coords: Vec<_> = tokenized_line[2].split(",").collect();
                let end_coords: Vec<_> = tokenized_line[4].split(",").collect();
                let start_x: usize = beginning_coords[0].parse().ok().expect("error");
                let start_y: usize = beginning_coords[1].parse().ok().expect("error");
                let end_x: usize = end_coords[0].parse().ok().expect("error");
                let end_y: usize = end_coords[1].parse().ok().expect("error");

                for i in start_x..end_x + 1 {
                    for j in start_y..end_y + 1 {
                        grid[i][j].turn_off();
                    }
                }
            }
        }
    }

    let mut brightness = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            brightness += grid[i][j].brightness;
        }
    }

    brightness
}
