pub fn get_solution_part1() -> i32 {
    let input = include_str!("day5input.txt");

    let mut nice_words = 0;

    for line in input.lines() {
        let mut vowels = 0;
        let mut double_letter = false;
        let mut prev_char: char = ' ';

        if line.contains("ab") || line.contains("cd") || line.contains("pq") ||
           line.contains("xy") {
            continue;
        }

        for ch in line.chars() {
            if ch == prev_char {
                double_letter = true;
            }

            match ch {
                'a' => {
                    vowels += 1;
                }
                'e' => {
                    vowels += 1;
                }
                'i' => {
                    vowels += 1;
                }
                'o' => {
                    vowels += 1;
                }
                'u' => {
                    vowels += 1;
                }
                _ => {}
            }

            prev_char = ch;
        }

        if vowels > 2 && double_letter {
            nice_words += 1;
        }
    }

    nice_words
}

pub fn get_solution_part2() -> i32 {
    let input = include_str!("day5input.txt");
    let mut nice_words = 0;

    for line in input.lines() {
        if two_chars(line) && palindrome_chars(line) {
            nice_words += 1;
        }
    }

    nice_words
}

fn two_chars(line: &str) -> bool {
    let chars: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(1).collect();

    let mut i = 0;
    let bounds = chars.len();
    for (char1, char2) in chars.iter().zip(chars2.iter()) {
        let j = i + 2;

        if j > bounds - 1 {
            break;
        }

        for n in j..(bounds - 1) {
            if &chars[n] == char1 && &chars[n + 1] == char2 {
                return true;
            }
        }

        i += 1;
    }

    false
}

fn palindrome_chars(line: &str) -> bool {
    let chars: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(2).collect();

    for (char1, char2) in chars.iter().zip(chars2.iter()) {
        if char1 == char2 {
            return true;
        }
    }

    false
}
