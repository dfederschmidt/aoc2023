use std::collections::HashMap;


fn first(line: &str) -> i32 {
    // Return the game id if its possible, zero otherwise
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let (game, games_str) = line.split_once(":").unwrap();
    let game_id = game.replace("Game ", "").parse::<i32>().unwrap();

    let games = games_str.split([',',';']);
    
    for entry in games {
        let (num_cubes, color) = entry.trim().split_once(" ").unwrap();
        let num_cubes = num_cubes.parse::<i32>().unwrap();

        match color {
            "green" => {
                if num_cubes > green_max {
                    return 0
                }
            }
            "red" => {
                if num_cubes > red_max {
                    return 0
                }
            }
            "blue" => {
                if num_cubes > blue_max {
                    return 0
                }
            }
            _ => panic!("Unknown color: {}", color)
        } 
        
    }

    game_id
}

fn second(line: &str) -> i32 {
    // Return the game id if its possible, zero otherwise

    let (game, games_str) = line.split_once(":").unwrap();
    let _game_id = game.replace("Game ", "").parse::<i32>().unwrap();

    let games = games_str.split([',',';']);
    
    let mut counters: HashMap<&str, i32> = HashMap::new();
    counters.insert("green", 0);
    counters.insert("red", 0);
    counters.insert("blue", 0);

    for entry in games {
        let (num_cubes, color) = entry.trim().split_once(" ").unwrap();
        let num_cubes = num_cubes.parse::<i32>().unwrap();

        if num_cubes > counters[color] {
            counters.insert(color, num_cubes);
        }
    }

    counters["green"] * counters["red"] * counters["blue"]
}


fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let first_result: i32 = lines.iter().map(|line| first(line)).sum();

    println!("{first_result}");


    let second_result: i32 = lines.iter().map(|line| second(line)).sum();
    println!("{}", second_result);

}
