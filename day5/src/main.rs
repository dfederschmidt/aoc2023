


fn process_map(ranges: &Vec<(i64, i64, i64)>, input: i64) -> i64 {

    let mut output = input;

    for (destination_range_start, source_range_start, range_length) in ranges {
        let source_range_end = source_range_start + range_length - 1;

        let input_is_in_range = source_range_start <= &input && input <= source_range_end;

        if input_is_in_range {
            let out = (input - source_range_start) + destination_range_start;
            //println!("in: {input} in range {source_range_start} <-> {source_range_end}: out: {out}");
            output = out
        }
    }
    
    return output
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<i64> = blocks[0]
                            .split_once(":")
                            .unwrap().1.trim()
                            .split(" ")
                            .map(|x| x.parse::<i64>().unwrap())
                            .collect();



    let mut location_values: Vec<i64> = vec![];

    let mut block_ranges: Vec<Vec<(i64, i64, i64)>> = vec![];

    for block in blocks {
        let mut block_lines: Vec<&str> = block.lines().collect();
        block_lines.remove(0);

        let ranges: Vec<(i64, i64, i64)> = block_lines.iter()
            .map(|line| {
                let nums = line.split_whitespace()
                                .map(|x| x.parse::<i64>().expect("Failed to parse integer"))
                                .collect::<Vec<_>>();
                let tuple = (nums[0], nums[1], nums[2]);
                tuple
            })
            .collect();

        block_ranges.push(ranges);
    }


    for seed in &seeds {
        let mut curr_value = *seed;

        for i in 1..=7 {
            curr_value = process_map(&block_ranges[i], curr_value)
        }

        location_values.push(curr_value);
    }

    println!("First {}", location_values.iter().min().unwrap());


    let mut min_location_value = 100000000000000000;

    let seed_ranges: Vec<_> = (0..seeds.len())
        .step_by(2)
        .filter_map(|i| seeds.get(i).and_then(|&x| seeds.get(i + 1).map(|&y| (x, y))))
        .collect();

    for (start, range) in seed_ranges {

        println!("{start}, {range}");

        for i in start..=start+range-1 {
            println!("Seed {i}");

            let mut curr_value = i;

            for block_i in 1..=7 {
                curr_value = process_map(&block_ranges[block_i], curr_value)
            }

            if curr_value < min_location_value {
                println!("new min {curr_value}");
                min_location_value = curr_value;
            }
        }

    }

    println!("Second {}", min_location_value);


}
