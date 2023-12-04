use std::collections::{HashSet, HashMap};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

fn first(line: &str) -> i32 {
    let (_card, card_content) = line.split_once(":").unwrap();
    let (winning_content, given_content) = card_content.split_once("|").unwrap();

    let winning_numbers: Vec<i32> = winning_content.split_whitespace().map(|el| {
        return el.parse::<i32>().unwrap();
    }).collect();

    let winning_numbers_set: HashSet<_> = HashSet::from_iter(winning_numbers);

    let given_numbers: Vec<i32> = given_content.split_whitespace().map(|el| {
        return el.parse::<i32>().unwrap();
    }).collect();

    let mut points = 0;

    for num in given_numbers {
        if winning_numbers_set.contains(&num) {

            if points == 0 {
                points += 1;
            }
            else {
                points *= 2;
            }
        }
    }
    
    return points
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Card {
    id: i32,
    num_matching: i32,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn parse_line_to_card(line: &str) -> Card {
    let (_card, card_content) = line.split_once(":").unwrap();
    let (winning_content, given_content) = card_content.split_once("|").unwrap();

    let winning_numbers: Vec<i32> = winning_content.split_whitespace().map(|el| {
        return el.parse::<i32>().unwrap();
    }).collect();

    let winning_numbers_set: HashSet<_> = HashSet::from_iter(winning_numbers);

    let given_numbers: Vec<i32> = given_content.split_whitespace().map(|el| {
        return el.parse::<i32>().unwrap();
    }).collect();

    let mut num_matching = 0;

    for num in given_numbers {
        if winning_numbers_set.contains(&num) {
            num_matching +=1
        }
    }

    Card {
        id: _card.strip_prefix("Card ").unwrap().trim().parse::<i32>().unwrap(),
        num_matching: num_matching,
    }

}

fn second (lines: Vec<&str>) -> i32 {

    let mut cards: HashMap<i32, Card> = HashMap::new();

    for line in &lines {
        let card = parse_line_to_card(line);
        cards.insert(card.id, card);
    }
    println!("Cards: {:?}", cards);

    let mut pq = BinaryHeap::new();

    for line in lines {
        let card = parse_line_to_card(line);
        pq.push(Reverse(card));
    }

    let mut num_cards = 0;

    while let Some(Reverse(card)) = pq.pop() {
        num_cards += 1;
        let card_id = card.id;

        for i in (card_id + 1)..=(card_id + card.num_matching) {
            pq.push(Reverse(cards[&i]));
        }
    }

    return num_cards;
}

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let first_result: i32 = lines.iter().map(|line| first(line)).sum();
    println!("first: {}", first_result);

    let second_result = second(lines);
    println!("second: {}", second_result);
}
