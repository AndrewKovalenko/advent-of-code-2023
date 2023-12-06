use std::u32;

use data_access::file::read_lines;
use regex::Regex;

const COLON: &str = ":";
const SEMICOLON: &str = ";";
const CAPTURE_GROUP_VALUE_NUMBER: usize = 1;

struct CubesMix {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

struct Game {
    id: u32,
    draws: Vec<String>,
}

fn too_many_cubes_in_the_draw(cubes_in_the_bag: u32, draw_line: &String, regex: &Regex) -> bool {
    if let Some(number_of_cubes_capture) = regex.captures(&draw_line) {
        let extracted_cubes_value = &number_of_cubes_capture[CAPTURE_GROUP_VALUE_NUMBER];

        let number_of_cubes = extracted_cubes_value.parse::<u32>().unwrap();

        if number_of_cubes > cubes_in_the_bag {
            return true;
        }
    }

    return false;
}

fn is_game_possible(bag: &CubesMix, draws_data: Vec<String>) -> bool {
    let blue_regex: Regex = Regex::new(r"\s(\d+) blue.*").unwrap();
    let red_regex: Regex = Regex::new(r"\s(\d+) red.*").unwrap();
    let green_regex: Regex = Regex::new(r"\s(\d+) green.*").unwrap();

    let impossible_game_found = draws_data.iter().find(|draw_line| {
        too_many_cubes_in_the_draw(bag.blue_cubes, *draw_line, &blue_regex)
            || too_many_cubes_in_the_draw(bag.green_cubes, *draw_line, &green_regex)
            || too_many_cubes_in_the_draw(bag.red_cubes, *draw_line, &red_regex)
    });

    impossible_game_found == None
}

fn parse_draws(input: String) -> Game {
    let game_id_regex = Regex::new(r"^Game (\d+).*").unwrap();
    let parsed_data: Vec<&str> = input.split(COLON).collect();
    let id_part = parsed_data[0];
    let draw_data = parsed_data[1];

    let draws = draw_data
        .split(SEMICOLON)
        .map(|line| String::from(line))
        .collect();
    let id = game_id_regex.captures(id_part).unwrap()[CAPTURE_GROUP_VALUE_NUMBER]
        .parse::<u32>()
        .unwrap();

    Game { draws, id }
}

fn main() {
    let bag = CubesMix {
        red_cubes: 12,
        green_cubes: 13,
        blue_cubes: 14,
    };

    // read_lines("game-possibility/data/input.txt")
    let number_of_impossible_games = read_lines("data/input.txt") // VsCode debug path
        .unwrap()
        .map(|line| parse_draws(line.unwrap()))
        .fold(0, |acc, game_data| {
            if is_game_possible(&bag, game_data.draws) {
                acc + game_data.id
            } else {
                acc
            }
        });

    println!("Number of possible games: {number_of_impossible_games}")
}
