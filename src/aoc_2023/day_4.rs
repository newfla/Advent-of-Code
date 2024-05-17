use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use rayon::{
    iter::{
        IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
    },
    str::ParallelString,
};
use regex::Regex;

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
    winning_count: u32,
}

impl Card {
    fn calculate_points(&mut self) -> i32 {
        let not_winning = self.numbers.difference(&self.winning_numbers).count() as u32;
        let winning_count = self.numbers.len() as u32 - not_winning;
        self.winning_count = winning_count;

        if not_winning == self.numbers.len() as u32 {
            0
        } else {
            2_i32.pow(winning_count - 1)
        }
    }

    fn next_cards(&self) -> Range<u32> {
        self.id + 1..self.winning_count + 1 + self.id
    }
}

fn parse_cards(data: &str) -> Vec<Card> {
    data.par_lines().map(parse_card).collect()
}

fn parse_card(line: &str) -> Card {
    let re = Regex::new(r"\d+").unwrap();
    let (id, all_numbers) = line.split_once(':').unwrap();
    let (winning_numbers, numbers) = all_numbers.split_once('|').unwrap();

    let id = re
        .find_iter(id)
        .map(|val| val.as_str().parse::<u32>().unwrap())
        .last()
        .unwrap();
    let winning_numbers = re
        .find_iter(winning_numbers)
        .map(|val| val.as_str().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    let numbers = re
        .find_iter(numbers)
        .map(|val| val.as_str().parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();
    Card {
        id,
        winning_numbers,
        numbers,
        winning_count: 0,
    }
}

#[allow(dead_code)]
fn day_4_part_1(data: &str) -> i32 {
    parse_cards(data)
        .par_iter_mut()
        .map(|card| card.calculate_points())
        .sum()
}

#[allow(dead_code)]
fn day_4_part_2(data: &str) -> u32 {
    let mut cards = parse_cards(data);
    let winning_map = cards
        .par_iter_mut()
        .map(|card| {
            card.calculate_points();
            (card.id, card)
        })
        .into_par_iter()
        .collect::<HashMap<_, _>>();

    winning_map
        .par_iter()
        .map(|(_id, card)| calculate(card.next_cards(), &winning_map))
        .sum()
}

fn calculate(range: Range<u32>, map: &HashMap<u32, &mut Card>) -> u32 {
    let mut accumulator = 1;
    for id in range {
        accumulator += calculate(map[&id].next_cards(), map);
    }
    accumulator
}

#[cfg(test)]
mod test {

    use super::{day_4_part_1, day_4_part_2};

    #[test]
    fn test_day_4_part_1() {
        let data = include_str!("../../data/aoc_2023/day_4.txt");

        let solution = day_4_part_1(data);
        println!("2023.4.1: {solution}");
    }

    #[test]
    fn test_day_4_part_2() {
        let data = include_str!("../../data/aoc_2023/day_4.txt");
        let solution = day_4_part_2(data);
        println!("2023.4.2: {solution}");
    }
}
