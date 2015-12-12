// How much wrapping paper do the elves need to order?
// Surface area plus slack (shortest side area)
pub fn part1_solution() -> i32 {
    let input = include_str!("day2input.txt");

    let mut total_surface_area = 0;

    for line in input.lines() {
        let dimensions = line.split("x").collect::<Vec<_>>();
        let length: i32 = dimensions[0].parse().ok().expect("Invalid integer");
        let width: i32 = dimensions[1].parse().ok().expect("Invalid integer");
        let height: i32 = dimensions[2].parse().ok().expect("Invalid integer");

        total_surface_area += calc_wrapping_paper_surface_area(length, width, height);
        total_surface_area += get_wrapping_paper_slack(length, width, height);
    }

    total_surface_area
}

// How much ribbon do the elves need to order?
// length + width * 2 plus cubic feet of gift for bow
pub fn part2_solution() -> i32 {
    let input = include_str!("day2input.txt");

    let mut total_ribbon = 0;

    for line in input.lines() {
        let dimensions = line.split("x").collect::<Vec<_>>();
        let length: i32 = dimensions[0].parse().ok().expect("Invalid integer");
        let width: i32 = dimensions[1].parse().ok().expect("Invalid integer");
        let height: i32 = dimensions[2].parse().ok().expect("Invalid integer");

        total_ribbon += get_total_ribbon_for_gift(length, width, height);
        total_ribbon += get_total_ribbon_for_bow(length, width, height);
    }

    total_ribbon
}

fn calc_wrapping_paper_surface_area(l: i32, w: i32, h: i32) -> i32 {
    let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;

    surface_area
}

fn get_wrapping_paper_slack(l: i32, w: i32, h: i32) -> i32 {
    let mut dimensions = [l, w, h];

    dimensions.sort();

    dimensions[0] * dimensions[1]
}

fn get_total_ribbon_for_gift(l: i32, w: i32, h: i32) -> i32 {
    let mut dimensions = [l, w, h];

    dimensions.sort();

    2 * (dimensions[0] + dimensions[1])
}

fn get_total_ribbon_for_bow(l: i32, w: i32, h: i32) -> i32 {
    l * w * h
}
