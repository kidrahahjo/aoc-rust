use std::{
    collections::{HashMap, HashSet},
    env, fs, io, process,
};

fn get_input_file_path_from_args() -> Result<String, &'static str> {
    env::args().nth(1).ok_or("Path to input file not provided")
}

fn read_lines_as_vec(filepath: String) -> Result<Vec<String>, io::Error> {
    let file_content = fs::read_to_string(filepath)?;
    Ok(file_content.lines().map(String::from).collect())
}

fn is_symbol(value: char) -> bool {
    !(value.is_numeric() || value == '.')
}

fn check_adjacent_symbol(
    current_line: &str,
    current_line_idx: usize,
    current_pos_idx: usize,
    lines: &[&String],
) -> (bool, Vec<(usize, usize)>) {
    let mut has_adjacent_symbol = false;
    let mut found: Vec<(usize, usize)> = vec![];
    let mut itervec: Vec<(usize, &str)> = vec![(current_line_idx, current_line)];

    if current_line_idx != 0 {
        itervec.push((current_line_idx - 1, lines[current_line_idx - 1]));
    }

    if current_line_idx != lines.len() - 1 {
        itervec.push((current_line_idx + 1, lines[current_line_idx + 1]));
    }

    for (line_idx, line) in itervec {
        if line.is_empty() {
            continue;
        }
        let mut chars = line.chars();
        // Check for left character
        if current_pos_idx != 0 {
            if let Some(value) = chars.nth(current_pos_idx - 1) {
                let is_this_symbol = is_symbol(value);
                has_adjacent_symbol = has_adjacent_symbol || is_this_symbol;

                if is_this_symbol {
                    found.push((line_idx, current_pos_idx - 1));
                }
            }
        }

        // Check for current character
        if let Some(value) = chars.next() {
            let is_this_symbol = is_symbol(value);
            has_adjacent_symbol = has_adjacent_symbol || is_this_symbol;

            if is_this_symbol {
                found.push((line_idx, current_pos_idx));
            }
        }

        // Check for next character
        if let Some(value) = chars.next() {
            let is_this_symbol = is_symbol(value);
            has_adjacent_symbol = has_adjacent_symbol || is_this_symbol;

            if is_this_symbol {
                found.push((line_idx, current_pos_idx + 1));
            }
        }
    }

    (has_adjacent_symbol, found)
}

fn solution_part1(#[allow(clippy::ptr_arg)] lines: &Vec<&String>) -> u64 {
    let mut res = 0u64;
    for (i, line) in lines.iter().enumerate() {
        let mut total = 0u64;
        let mut current_number = String::new();
        let mut has_adjacent_symbol = false;

        line.chars().enumerate().for_each(|(j, char)| {
            let digit = char.to_digit(10);
            match digit {
                Some(value) => {
                    current_number.push_str(&value.to_string());
                    let (curr_has_adjacent, _) = check_adjacent_symbol(line, i, j, lines);
                    has_adjacent_symbol = has_adjacent_symbol || curr_has_adjacent
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

fn solution_part2(#[allow(clippy::ptr_arg)] lines: &Vec<&String>) -> u128 {
    let mut part_number_tracker: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let mut current_number = String::new();
        let mut has_adjacent_symbol = false;
        let mut curr_found: HashSet<(usize, usize)> = HashSet::new();

        line.chars().enumerate().for_each(|(j, char)| {
            let digit = char.to_digit(10);
            match digit {
                Some(value) => {
                    current_number.push_str(&value.to_string());
                    let (curr_has_adjacent, curr_found_pairs) =
                        check_adjacent_symbol(line, i, j, lines);

                    curr_found_pairs.into_iter().for_each(|pair| {
                        curr_found.insert(pair);
                    });

                    has_adjacent_symbol = has_adjacent_symbol || curr_has_adjacent
                }
                None => {
                    if has_adjacent_symbol {
                        let current_number_parsed = current_number.parse::<u64>().unwrap_or(0);
                        for (pair_i, pair_j) in &curr_found {
                            part_number_tracker
                                .entry((*pair_i, *pair_j))
                                .or_default()
                                .push(current_number_parsed);
                        }
                        curr_found.clear();
                        has_adjacent_symbol = false;
                    }
                    current_number.clear();
                }
            };
        });

        if has_adjacent_symbol {
            let current_number_parsed = current_number.parse::<u64>().unwrap_or(0);
            for (pair_i, pair_j) in &curr_found {
                part_number_tracker
                    .entry((*pair_i, *pair_j))
                    .or_default()
                    .push(current_number_parsed);
            }
            curr_found.clear();
        }
    }

    part_number_tracker.values().fold(0u128, |res, value| {
        let mut mul = 0u128;
        if value.len() == 2 {
            mul = value[0] as u128 * value[1] as u128;
        }
        res + mul
    })
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
