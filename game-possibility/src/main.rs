use std::env::current_dir;

use data_access::file::read_lines;
use regex::Regex;

const COLON: &str = ":";
const SEMICOLON: &str = ";";

struct CubesMix {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

fn is_game_possible(bag: CubesMix, draws_data: Vec<String>) -> bool {
    let blue_regex: Regex = Regex::new(r".*(\d) blue.*").unwrap();
    let red_regex: Regex = Regex::new(r".*(\d) red.*").unwrap();
    let GREEN_REGEX: Regex = Regex::new(r".*(\d) green.*").unwrap();

    draws_data.iter().map(|draw_line| {
        let mut blue_cubes: u32 = 0;
        if let Some(blue_value) = blue_regex.captures(draw_line) {
            let one = &blue_value[1];

            blue_cubes = one.parse::<u32>().unwrap();
        }
    });

    false
}

fn parse_draws(input: String) -> Vec<String> {
    let parsed_data: Vec<&str> = input.split(COLON).collect();
    let game_data = parsed_data[1];

    game_data
        .split(SEMICOLON)
        .map(|line| String::from(line))
        .collect()
}

fn main() {
    // read_lines("game-possibility/data/input.txt")
    read_lines("data/input.txt")
        .unwrap()
        .map(|line| parse_draws(line.unwrap()))
        .for_each(|draw_data| {
            let bag = CubesMix {
                red_cubes: 12,
                green_cubes: 13,
                blue_cubes: 14,
            };

            let game_is_possible = is_game_possible(bag, draw_data);
        });
}
