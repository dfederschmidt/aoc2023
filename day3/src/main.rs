use std::collections::HashMap;

fn _print_schematic (schematic: &Vec<Vec<char>>) {
    for row in schematic {
        for ch in row {
            print!("{} ", ch);
        }
        print!("\n");
    }
}

fn first (schematic: &Vec<Vec<char>>) -> i32{

    let mut sum_of_part_numbers = 0;
    let mut current_number = String::new();
    let mut current_number_is_part = false;

    let is_part = |x: usize, y: usize| {
        let mut num_adjacent_parts = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbour_x = x as isize + i;
                let neighbour_y = y as isize + j;

                match (neighbour_x, neighbour_y) {
                    (r, c) if r >= 0 && c >= 0 && r < schematic.len() as isize && c < schematic[0].len() as isize => {
                        let neighbour = schematic[r as usize][c as usize];
                        
                        if !neighbour.is_alphabetic() && !neighbour.is_numeric() && neighbour != '.' {
                            num_adjacent_parts += 1;
                        }
                    }
                    _ => {
                        // Out of bounds, handle accordingly
                        continue;
                    }
    
                }
            }
        }
        
        if num_adjacent_parts > 0 {
            return true
        }
        false
    };

    for (row_index, row) in schematic.iter().enumerate() {

        for (col_index, ch) in row.iter().enumerate() {

            if ch.is_numeric() {
                current_number.push(*ch);

                if is_part(row_index, col_index) {
                    current_number_is_part = true;
                }
            } 
            if (!ch.is_numeric() && current_number.len() > 0) || (col_index == schematic[0].len() && ch.is_numeric()) {

                if current_number_is_part {
                    sum_of_part_numbers += current_number.parse::<i32>().unwrap();
                } 
                current_number.clear();
                current_number_is_part = false;
            }
        
        }
    }
    return sum_of_part_numbers;
}


fn second (schematic: &Vec<Vec<char>>) -> i32{

    let mut current_number = String::new();
    let mut gears: HashMap<String, Vec<i32>> = HashMap::new();

    let is_part = |x: usize, y: usize| {

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let neighbour_x = x as isize + i;
                let neighbour_y = y as isize + j;

                match (neighbour_x, neighbour_y) {
                    (r, c) if r >= 0 && c >= 0 && r < schematic.len() as isize && c < schematic[0].len() as isize => {
                        let neighbour = schematic[r as usize][c as usize];
                        
                        if !neighbour.is_alphabetic() && !neighbour.is_numeric() && neighbour != '.' {
                            return Some((neighbour, neighbour_x, neighbour_y));
                        }
                    }
                    _ => {
                        // Out of bounds, handle accordingly
                        continue;
                    }
    
                }
            }
        }
        return None
    };

    for (row_index, row) in schematic.iter().enumerate() {
        let mut current_gear = String::new();

        for (col_index, ch) in row.iter().enumerate() {

            if ch.is_numeric() {
                current_number.push(*ch);
                

                match is_part(row_index, col_index) {
                    Some((value, x, y)) => {

                        if value == '*' {
                            current_gear = format!("{x},{y}");
                        }
                    },
                    None => ()
                }
            } 
            if (!ch.is_numeric() && current_number.len() > 0) || (col_index == schematic[0].len() - 1 && ch.is_numeric()) {
                
                if current_gear.len() > 0 {
                    //println!("{current_number} is adjacent to gear");
                    let part_number = current_number.parse::<i32>().unwrap();
                    gears.entry(current_gear.clone()).or_insert_with(Vec::new).push(part_number);
                    current_gear.clear();
                } else {
                    //println!("{current_number} is not adjacent to gear");
                }
                current_number.clear();
            }
        
        }
    }

    //println!("Gears: {:#?}", gears);

    let mut gear_ratio_sum = 0;

    for (_key, value) in gears.iter_mut() {
        if value.len() == 2 {
            //println!("Gear= {} * {}", value[0], value[1]);
            gear_ratio_sum += value[0] * value[1];
        }
    }

    //println!("{}", gears.len());
    return gear_ratio_sum
}


fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let chars = line.chars().collect();
        schematic.push(chars);
    }

    let first_result = first(&schematic);
    println!("First: {first_result}");


    let second_result = second(&schematic);
    println!("Second: {second_result}")
}
