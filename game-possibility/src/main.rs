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

fn parse_draws(input: String) -> Vec<String> {
    let parsed_data: Vec<&str> = input.split(COLON).collect();
    let game_data = parsed_data[1];

    game_data
        .split(SEMICOLON)
        .map(|line| String::from(line))
        .collect()
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
        .fold(0, |acc, draw_data| {
            if !is_game_possible(&bag, draw_data) {
                acc + 1
            } else {
                acc
            }
        });

    println!("Number of impossible games: {number_of_impossible_games}")
}
