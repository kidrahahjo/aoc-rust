use std::{collections::HashMap, env, fs, io, process};

fn get_input_file_path_from_args() -> Result<String, &'static str> {
    env::args().nth(1).ok_or("Path to input file not provided")
}

fn read_lines_as_vec(filepath: String) -> Result<Vec<String>, io::Error> {
    let file_content = fs::read_to_string(filepath)?;
    Ok(file_content.lines().map(String::from).collect())
}

fn get_game_number(line: &str) -> u64 {
    let number_details: Vec<&str> = line.split(" ").collect();
    let parsed_number = number_details[1].parse::<u64>();
    match parsed_number {
        Ok(value) => value,
        Err(_) => {
            println!("Something went wrong while processing the input file");
            process::exit(1);
        }
    }
}

fn parse_colour_details(game_detail_str: &str) -> Option<(&str, u64)> {
    let mut details_itr = game_detail_str.split(" ").into_iter();
    let number = details_itr.next().unwrap_or("");
    let colour = details_itr.next().unwrap_or("");
    if number == "" || colour == "" {
        return None;
    }
    match number.parse::<u64>() {
        Ok(value) => Some((colour, value)),
        Err(_) => None,
    }
}

fn get_colour_map(red: u64, blue: u64, green: u64) -> HashMap<&'static str, u64> {
    HashMap::from_iter(vec![("red", red), ("blue", blue), ("green", green)])
}

fn solution_part1(lines: Vec<&String>) -> u64 {
    let max_configurations = get_colour_map(12, 14, 13);

    lines.into_iter().fold(0, |res, line| {
        let mut line_split = line.split(": ");

        let game_number: u64 = line_split.next().map_or_else(
            || {
                println!("No game number provided");
                process::exit(1);
            },
            get_game_number,
        );
        res + match line_split.next() {
            Some(game_data) => {
                let success = game_data.split("; ").into_iter().all(|round| {
                    let mut round_map = get_colour_map(0, 0, 0);

                    round.split(", ").into_iter().for_each(|value| {
                        if let Some((colour, total)) = parse_colour_details(value) {
                            round_map.insert(colour, round_map.get(colour).unwrap_or(&0) + total);
                        }
                    });

                    max_configurations
                        .iter()
                        .all(|(key, value)| round_map.get(*key).unwrap_or(&0) <= value)
                });

                if success { game_number } else { 0u64 }
            }
            None => 0u64,
        }
    })
}

fn solution_part2(lines: Vec<&String>) -> u64 {
    lines.into_iter().fold(0, |res, line| {
        let line_split = line.split(": ").nth(1);

        res + match line_split {
            Some(game_data) => {
                let mut max_configurations = get_colour_map(0, 0, 0);
                game_data.split("; ").into_iter().for_each(|round| {
                    round.split(", ").into_iter().for_each(|value| {
                        if let Some((colour, total)) = parse_colour_details(value) {
                            if max_configurations.get(colour).unwrap_or(&0) < &total {
                                max_configurations.insert(colour, total);
                            }
                        }
                    });
                });
                let mut power = max_configurations.values().fold(1, |mut inner_res, value| {
                    if value != &0 {
                        inner_res *= value;
                    }
                    inner_res
                });

                if max_configurations.values().all(|value| value == &0) {
                    power = 0;
                }
                power
            }
            None => 0u64,
        }
    })
}

// Link: https://adventofcode.com/2023/day/2
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
