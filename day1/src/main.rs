extern crate regex;

use regex::{Regex};

fn get_calibration_value(digits: String) -> i32{
    return format!("{}{}", digits.chars().next().unwrap(), digits.chars().last().unwrap()).parse::<i32>().unwrap();
}

fn first(line: &str) -> i32 {
    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
    return get_calibration_value(digits);
}

fn second(line: &str) -> i32 {

    let num_regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut digits = String::new();

    for (index, character) in line.char_indices() {
        
        match character.is_digit(10) {
            true => {
                digits.push(character);
            }
            false => {
                let remaining_string =  line.get(index..).unwrap_or("");

                if let Some(caps) = num_regex.captures(remaining_string) {
                    if let Some(matched) = caps.get(1) {
                        let matched_str = matched.as_str();
                        if matched.start() == 0  {
                            let result = match matched_str {
                                "one" => "1",
                                "two" => "2",
                                "three" => "3",
                                "four" => "4",
                                "five" => "5",
                                "six" => "6",
                                "seven" => "7",
                                "eight" => "8",
                                "nine" => "9",
                                _ => panic!("Unexpected match: {}", matched_str)
                            };
                            digits += result;
                        }
                    }
                }            
            }
        }
    }

    return get_calibration_value(digits);
}

fn main() {

    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let first_result: i32 = lines.iter().map(|line| first(line)).sum();
    println!("{}", first_result);

    let second_result: i32 = lines.iter().map(|line| second(line)).sum();
    println!("{}", second_result);

}
