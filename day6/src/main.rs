fn calc_ways_to_beat_record(race: (i128, i128)) -> i128 {
    let time = race.0;

    let mut num_ways: i128 = 0;
    
    for button_press_time in 0..time {
        if button_press_time * (time - button_press_time) > race.1 {
            num_ways += 1
        }
    }


    return num_ways;
}

fn main() {
    let input: String = std::fs::read_to_string("src/input.txt").unwrap();
    let mut lines = input.lines();

    let times_line = lines.next().unwrap();
    let times: Vec<i128> = times_line.trim()
                            .split_whitespace()
                            .filter_map(|s| s.parse::<i128>().ok())
                            .collect();
    
    let distance_line = lines.next().unwrap();
    let distances: Vec<i128> = distance_line.trim()
                            .split_whitespace()
                            .filter_map(|s| s.parse::<i128>().ok())
                            .collect();


    let race_document:Vec<(i128,i128)> = times.iter()
                                .zip(distances.iter())
                                .map(|(&time, &distance)| (time, distance))
                                .collect();



    let second_time = times.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("").parse::<i128>().unwrap();

    let second_distance = distances.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("").parse::<i128>().unwrap();


    let mut product = 1;

    for race in race_document {
        product *= calc_ways_to_beat_record(race);
    }

    println!("first: {product}");

    println!("second: {}", calc_ways_to_beat_record((second_time, second_distance)))

}
