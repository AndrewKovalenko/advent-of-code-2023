use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_number(input: &str) -> u8 {
    let mut first_digit: Option<u8> = None;
    let mut second_digit: Option<u8> = None;
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        if first_digit == None {
            let leading_char = chars[i];

            if leading_char.is_numeric() {
                first_digit = Some(leading_char.to_digit(10).unwrap() as u8)
            }
        }

        if second_digit == None {
            let trailing_char = chars[input.len() - 1 - i];

            if trailing_char.is_numeric() {
                second_digit = Some(trailing_char.to_digit(10).unwrap() as u8)
            }
        }
    }

    first_digit.unwrap() * 10 + second_digit.unwrap()
}

fn main() {
    let resulted_sum = read_lines("../data/input.txt")
        .unwrap()
        .fold(0, |acc, element| {
            let number_extracted_from_line = extract_number(element.unwrap().as_str());
            acc + number_extracted_from_line
        });

    println!("Calculated sum of all lines is {resulted_sum}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_extraction() {
        let input42 = "asdf4sdfg2lkj";
        let result42 = extract_number(input42);

        assert_eq!(result42, 42);

        let input55 = "asdf5sdfgolkj";
        let result55 = extract_number(input55);

        assert_eq!(result55, 55);
    }
}
