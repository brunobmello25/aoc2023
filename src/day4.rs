use std::fs;

pub fn run() {
    run_part_2();
}

#[derive(Debug, Clone)]
struct Card {
    #[allow(dead_code)]
    id: usize,
    my_numbers: Vec<usize>,
    wining_numbers: Vec<usize>,
}

impl Card {
    fn get_points(&self) -> usize {
        let mut wins = 0;
        for my_number in &self.my_numbers {
            if self.wining_numbers.contains(my_number) {
                wins += 1;
            }
        }

        if wins == 0 {
            return 0;
        }

        return 2usize.pow((wins - 1) as u32);
    }

    fn get_points_part_2(&self) -> usize {
        let mut wins = 0;
        for my_number in &self.my_numbers {
            if self.wining_numbers.contains(my_number) {
                wins += 1;
            }
        }

        return wins;
    }
}

#[allow(dead_code)]
fn run_part_1() {
    let contents = fs::read_to_string("input/day4.txt").unwrap();
    let contents = contents.trim();

    let cards: Vec<_> = contents
        .split("\n")
        .map(|line| line_to_card(line.to_string()))
        .collect();

    let total_points = cards.iter().map(|card| card.get_points()).sum::<usize>();
    println!("Total points: {}", total_points);
}

#[allow(dead_code)]
fn run_part_2() {
    let contents = fs::read_to_string("input/day4.txt").unwrap();
    let contents = contents.trim();

    let cards: Vec<_> = contents
        .split("\n")
        .map(|line| line_to_card(line.to_string()))
        .collect();

    let mut amount = cards.len();
    let mut won_copies: Vec<Card> = vec![];

    for card in &cards {
        let copies: Vec<_> = (0..card.get_points_part_2())
            .map(|n| n + card.id + 1)
            .map(|id| cards.iter().find(|c| c.id == id).unwrap().clone())
            .collect();
        won_copies.extend(copies);
    }
    amount += won_copies.len();

    while let Some(card) = won_copies.pop() {
        let copies: Vec<_> = (0..card.get_points_part_2())
            .map(|n| n + card.id + 1)
            .map(|id| cards.iter().find(|c| c.id == id).unwrap().clone())
            .collect();
        amount += copies.len();
        won_copies.extend(copies);
    }

    println!("Total amount: {}", amount);
    // while won_copies has elements, take one and possibily add new copies to won_copies
    // repeat this process until it's empty
}

fn line_to_card(line: String) -> Card {
    let parts: Vec<_> = line.split(": ").map(|part| part.trim()).collect();
    let id = parts
        .get(0)
        .unwrap()
        .replace("Card ", "")
        .trim()
        .parse::<usize>()
        .unwrap();

    let [wining_numbers, my_numbers] =
        match parts.get(1).unwrap().split(" | ").collect::<Vec<_>>()[..] {
            [wining_numbers, my_numbers] => [wining_numbers, my_numbers],
            _ => panic!("Invalid line"),
        };
    let wining_numbers: Vec<_> = wining_numbers
        .split(" ")
        .filter(|n| n != &"")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let my_numbers: Vec<_> = my_numbers
        .split(" ")
        .filter(|n| n != &"")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    return Card {
        id,
        my_numbers,
        wining_numbers,
    };
}

#[cfg(test)]
mod tests {}
