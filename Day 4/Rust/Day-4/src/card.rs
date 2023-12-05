pub struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    pub matches: Vec<i32>,
    instances: i32
}

impl Card {
    pub fn new(id: i32, winning_numbers: Vec<i32>, numbers: Vec<i32>) -> Card {
        Card {
            id,
            winning_numbers,
            numbers,
            matches: Vec::new(),
            instances: 1
        }
    }

    pub fn check_matches(&mut self) {
        let mut matches = Vec::new();
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                matches.push(*number);
            }
        }
        self.matches = matches;
    }
}
