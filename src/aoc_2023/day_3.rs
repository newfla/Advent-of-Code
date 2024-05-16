use std::ops::Range;

use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

struct Part {
    number: u32,
    x: Range<i32>,
    y: Range<i32>,
}

impl Part {
    fn filter(&self, symbols: &[Symbol]) -> Option<&Self> {
        if symbols
            .par_iter()
            .any(|symbol| self.x.contains(&symbol.x) && self.y.contains(&symbol.y))
        {
            Some(self)
        } else {
            None
        }
    }
}

struct Symbol {
    x: i32,
    y: i32,
}

impl Symbol {
    fn gear_ratio(&self, parts: &[Part]) -> Option<u32> {
        let parts = parts
            .par_iter()
            .filter(|part| part.x.contains(&self.x) && part.y.contains(&self.y))
            .collect::<Vec<&Part>>();
        if parts.len() == 2 {
            Some(parts[0].number * parts[1].number)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn day_3_part_1(data: &str) -> u32 {
    let parts = ugly_parts_parser(data);
    let symbols = ugly_symbols_parser(data);

    parts
        .par_iter()
        .filter_map(|part| part.filter(&symbols))
        .fold_with(0, |acc, elem| acc + elem.number)
        .sum()
}

#[allow(dead_code)]
fn day_3_part_2(data: &str) -> u32 {
    let parts = ugly_parts_parser(data);
    let maybe_gears = ugly_gears_parser(data);
    maybe_gears
        .par_iter()
        .filter_map(|symbol| symbol.gear_ratio(&parts))
        .sum()
}

fn ugly_parts_parser(data: &str) -> Vec<Part> {
    data.lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            let mut numbers = Vec::new();
            let mut accumulator = "".to_owned();
            let mut start = None;

            for (char_idx, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    accumulator += &char.to_string();
                    if start.is_none() {
                        start = Some(char_idx);
                    }
                } else if let Some(val) = parse_part(&accumulator, idx, char_idx, &start) {
                    numbers.push(val);
                    start = None;
                    accumulator = "".to_string();
                }
            }
            if start.is_some() {
                if let Some(val) = parse_part(&accumulator, idx, line.len(), &start) {
                    numbers.push(val);
                }
            }
            numbers
        })
        .collect()
}

fn parse_part(
    accumulator: &str,
    idx: usize,
    char_idx: usize,
    start: &Option<usize>,
) -> Option<Part> {
    if let Ok(val) = accumulator.parse::<u32>() {
        Some(Part {
            number: val,
            x: idx as i32 - 1..idx as i32 + 2,
            y: start.unwrap() as i32 - 1..char_idx as i32 + 1, //we are already +1 the end of number
        })
    } else {
        None
    }
}

fn ugly_symbols_parser(data: &str) -> Vec<Symbol> {
    data.lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            //let mut symbols = Vec::new();
            line.par_char_indices()
                .filter_map(|(sym_idx, sym)| {
                    if sym.is_ascii_punctuation() && sym != '.' {
                        Some(Symbol {
                            x: idx as i32,
                            y: sym_idx as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Symbol>>()
        })
        .collect()
}

fn ugly_gears_parser(data: &str) -> Vec<Symbol> {
    data.lines()
        .enumerate()
        .flat_map(|(idx, line)| {
            //let mut symbols = Vec::new();
            line.par_char_indices()
                .filter_map(|(sym_idx, sym)| {
                    if sym == '*' {
                        Some(Symbol {
                            x: idx as i32,
                            y: sym_idx as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Symbol>>()
        })
        .collect()
}

#[cfg(test)]
mod test {

    use super::{day_3_part_1, day_3_part_2};

    #[test]
    fn test_day_3_part_1() {
        let data = include_str!("../../data/aoc_2023/day_3.txt");

        let solution = day_3_part_1(data);
        println!("2023.3.1: {solution}");
    }

    #[test]
    fn test_day_3_part_2() {
        let data = include_str!("../../data/aoc_2023/day_3.txt");
        let solution = day_3_part_2(data);
        println!("2023.3.2: {solution}");
    }
}
