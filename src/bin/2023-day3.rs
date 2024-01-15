use std::{env, fs, io, process};

fn get_input_file_path_from_args() -> Result<String, &'static str> {
    env::args().nth(1).ok_or("Path to input file not provided")
}

fn read_lines_as_vec(filepath: String) -> Result<Vec<String>, io::Error> {
    let file_content = fs::read_to_string(filepath)?;
    Ok(file_content.lines().map(String::from).collect())
}

fn is_symbol(value: &char) -> bool {
    !(value.is_numeric() || value.to_string() == ".")
}

fn check_adjacent_symbol(
    current_line: &str,
    current_idx: usize,
    previous_line: &str,
    next_line: &str,
) -> bool {
    let mut has_adjacent_symbol = false;
    for line in [current_line, previous_line, next_line] {
        if line == "" {
            continue;
        }
        let mut chars = line.chars();
        // Check for left character
        if current_idx != 0 {
            if let Some(value) = chars.nth(current_idx - 1) {
                has_adjacent_symbol = has_adjacent_symbol || is_symbol(&value);
            }
        }

        // Check for current character
        if let Some(value) = chars.next() {
            has_adjacent_symbol = has_adjacent_symbol || is_symbol(&value);
        }

        // Check for next character
        if let Some(value) = chars.next() {
            has_adjacent_symbol = has_adjacent_symbol || is_symbol(&value);
        }

        if has_adjacent_symbol {
            return has_adjacent_symbol;
        }
    }
    has_adjacent_symbol
}

fn solution_part1(lines: &Vec<&String>) -> u64 {
    let mut res = 0u64;
    for (i, line) in lines.into_iter().enumerate() {
        let mut total = 0u64;
        let mut current_number = String::new();
        let mut has_adjacent_symbol = false;
        line.chars().enumerate().for_each(|(j, char)| {
            let digit = char.to_digit(10);
            match digit {
                Some(value) => {
                    let mut previous = "";
                    if i != 0 {
                        previous = lines[i - 1];
                    }

                    let mut next = "";
                    if i != lines.len() - 1 {
                        next = lines[i + 1];
                    }

                    current_number.push_str(&value.to_string());
                    has_adjacent_symbol =
                        has_adjacent_symbol || check_adjacent_symbol(line, j, previous, next);
                }
                None => {
                    if has_adjacent_symbol {
                        total += current_number.parse::<u64>().unwrap_or(0);
                        has_adjacent_symbol = false;
                    }
                    current_number.clear();
                }
            };
        });

        if has_adjacent_symbol {
            total += current_number.parse::<u64>().unwrap_or(0);
        }

        res += total;
    }
    res
}

fn solution_part2(_lines: &Vec<&String>) -> u128 {
    0
}

// Link: https://adventofcode.com/2023/day/3
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
            println!("Failed to read file");
            process::exit(1);
        }
    };

    println!("Solution for part 1 is {}", solution_part1(&lines.iter().collect()));
    println!("Solution for part 2 is {}", solution_part2(&lines.iter().collect()));
}
