mod card;
use card::Card;

fn main() {
    // Read in the file
    let input = include_str!("../../input.txt");

    // Split the input into lines
    let lines: Vec<&str> = input.split("\n").collect();

    let cards = extract_cards(lines);

    println!("Part 1: {}", part_1_result(cards));

}

fn extract_cards(lines: Vec<&str>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        // Skip empty lines
        if line == "" {
            continue;
        }

        // ID is between the first space and the colon
        let id: i32 = line[line.find(" ").unwrap() + 1..line.find(":").unwrap()].trim().parse().unwrap();
        // winning_numbers are between the colon and the pipe
        let winning_numbers: Vec<i32> = line[line.find(":").unwrap() + 1..line.find("|").unwrap()].trim().replace("  ", " ")
                                        .split(" ").map(|x| x.parse().unwrap()).collect();
        // numbers are between the pipe and the end of the line
        let numbers: Vec<i32> = line[line.find("|").unwrap() + 1..].trim().replace("  ", " ")
                                .split(" ").map(|x| x.parse().unwrap()).collect();
        cards.push(Card::new(id, winning_numbers, numbers));

        // set the matches for the card
        cards.last_mut().unwrap().check_matches();
    }

    return cards;
}

fn part_1_result(cards: Vec<Card>) -> i32 {
    let mut total: Vec<i32> = Vec::new();
    for card in cards {
        // if the card has no matches, skip it
        if card.matches.len() == 0 {
            continue;
        }

        // add 1 * 2 ^ len(matches) - 1 to the total
        total.push(1 * 2_i32.pow((card.matches.len() as u32) - 1));
    }

    // return the sum of the total
    return total.iter().sum();
}

// What have I learned?
// - _<type> after an immediate value will cast it to that type
// - the .map function is nice for doing matrix-style functional operations
// - delegates/anonymous functions have a different syntax than typical - |x| x + 1 instead of (x) => x + 1