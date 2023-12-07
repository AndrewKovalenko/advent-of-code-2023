use once_cell::sync::Lazy;
use std::u32;

use data_access::file::read_lines;
use regex::Regex;

const COLON: &str = ":";
const SEMICOLON: &str = ";";
const CAPTURE_GROUP_VALUE_NUMBER: usize = 1;

static BLUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s(\d+) blue.*").unwrap());
static RED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s(\d+) red.*").unwrap());
static GREEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s(\d+) green.*").unwrap());

struct CubesMix {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

struct Game {
    id: u32,
    draws: Vec<String>,
}

fn extract_number_of_cubes(draw_line: &String, regex: &Regex) -> u32 {
    if let Some(number_of_cubes_capture) = regex.captures(&draw_line) {
        let extracted_cubes_value = &number_of_cubes_capture[CAPTURE_GROUP_VALUE_NUMBER];

        return extracted_cubes_value.parse::<u32>().unwrap();
    }

    0
}

fn is_game_possible(bag: &CubesMix, draws_data: Vec<String>) -> bool {
    let impossible_game_found = draws_data.iter().find(|draw_line| {
        extract_number_of_cubes(draw_line, &BLUE_REGEX) > bag.blue_cubes
            || extract_number_of_cubes(draw_line, &RED_REGEX) > bag.red_cubes
            || extract_number_of_cubes(draw_line, &GREEN_REGEX) > bag.green_cubes
    });

    impossible_game_found == None
}

fn get_max_number_of_cubes(draw_data: Vec<String>) -> (u32, u32, u32) {
    let (mut red_cubes, mut blue_cubes, mut green_cubes) = (0, 0, 0);

    for draw_line in draw_data {
        let red_cubes_in_the_draw = extract_number_of_cubes(&draw_line, &RED_REGEX);
        let blue_cubes_in_the_draw = extract_number_of_cubes(&draw_line, &BLUE_REGEX);
        let green_cubes_in_the_draw = extract_number_of_cubes(&draw_line, &GREEN_REGEX);

        if red_cubes < red_cubes_in_the_draw {
            red_cubes = red_cubes_in_the_draw
        }

        if blue_cubes < blue_cubes_in_the_draw {
            blue_cubes = blue_cubes_in_the_draw
        }

        if green_cubes < green_cubes_in_the_draw {
            green_cubes = green_cubes_in_the_draw
        }
    }

    (red_cubes, green_cubes, blue_cubes)
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

    println!("[First part] Number of possible games: {number_of_impossible_games}");

    let power = read_lines("data/input.txt") // VsCode debug path
        .unwrap()
        .map(|line| parse_draws(line.unwrap()))
        .fold(0, |acc, game_data| {
            let (red, blue, green) = get_max_number_of_cubes(game_data.draws);

            acc + (red * blue * green)
        });

    println!("[Second part] Power of minimal cubes: {power}");
}
