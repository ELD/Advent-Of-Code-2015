use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn get_solution_part1() -> u64 {
    let secret_key = "yzbqklnj".as_bytes();
    let mut hasher = Md5::new();
    let mut number = 0;

    for i in 1..!0 as u64 {
        hasher.input(secret_key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;

        if first_five == 0 {
            number = i;
            break;
        }

        hasher.reset();
    }

    number
}

pub fn get_solution_part2() -> u64 {
    let secret_key = "yzbqklnj".as_bytes();
    let mut hasher = Md5::new();
    let mut number = 0;

    for i in 1..!0 as u64 {
        hasher.input(secret_key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_six = output[0] as i32 + output[1] as i32 + output[2] as i32;

        if first_six == 0 {
            number = i;
            break;
        }

        hasher.reset();
    }

    number
}
