use std::{
    collections::{hash_map::RandomState, HashMap},
    env, fs, io, process,
};

fn get_input_file_path_from_args() -> Result<String, &'static str> {
    let env_args: Vec<String> = env::args().collect();
    if env_args.len() < 2 {
        return Err("Path to input file not provided");
    }
    Ok(env_args[1].to_string())
}

fn read_lines_as_vec(filepath: String) -> Result<Vec<String>, io::Error> {
    let file_content = fs::read_to_string(filepath)?;
    Ok(file_content.lines().map(String::from).collect())
}

fn solution_part1(lines: Vec<&String>) -> u64 {
    const MAX_DIGIT_OVERFLOW: u64 = 10;
    lines.iter().fold(0u64, |mut res, line| {
        let (mut first, mut second) = (MAX_DIGIT_OVERFLOW, MAX_DIGIT_OVERFLOW);
        for char in line.chars() {
            let number_opt = char.to_digit(10);
            if let Some(number) = number_opt {
                let number_into_u64: u64 = number.into();
                if first == 10 {
                    res += number_into_u64 * 10;
                    first = number_into_u64;
                } else {
                    second = number_into_u64;
                }
            }
        }

        if second == MAX_DIGIT_OVERFLOW && first != MAX_DIGIT_OVERFLOW {
            second = first;
        }

        if second != MAX_DIGIT_OVERFLOW {
            res += second;
        }
        res
    })
}

fn solution_part2(lines: Vec<&String>) -> u64 {
    const WORD_PAIR: [(&str, u32); 20] = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let word_index: HashMap<&str, u32, RandomState> = HashMap::from_iter(WORD_PAIR);

    lines.iter().fold(0u64, |mut res, line| {
        let mut first: Option<&str> = None;
        let mut second: Option<&str> = None;
        for i in 0..line.len() {
            let mut matched_word_from_current: Option<&str> = None;
            for j in 1..6usize {
                if let Some(word) = line.get(i..i + j) {
                    if word_index.contains_key(word) {
                        matched_word_from_current = Some(word);
                        break;
                    }
                };
            }

            if let Some(word) = matched_word_from_current {
                if first.is_none() {
                    first = Some(word);
                }
                second = Some(word);
            }
        }

        if let Some(matched_word) = first {
            let number_into_u64: u64 = (*word_index.get(matched_word).unwrap()).into();
            res += number_into_u64 * 10;
        }
        if let Some(matched_word) = second {
            let number_into_u64: u64 = (*word_index.get(matched_word).unwrap()).into();
            res += number_into_u64;
        }

        res
    })
}

// Link: https://adventofcode.com/2023/day/1
fn main() {
    let filepath = match get_input_file_path_from_args() {
        Ok(path) => path,
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    };

    let lines = match read_lines_as_vec(filepath) {
        Ok(lines) => lines,
        Err(_) => {
            print!("Failed to read file");
            process::exit(1);
        }
    };

    println!("Solution for part 1 is {}", solution_part1(lines.iter().collect()));
    println!("Solution for part 2 is {}", solution_part2(lines.iter().collect()));
}
