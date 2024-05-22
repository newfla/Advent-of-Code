
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_combo(&self) -> u64 {
        let discriminant = (self.time * self.time - 4 * self.distance) as f64;
        let sqrt_discriminant = discriminant.sqrt();
        let min_hold_time = (((self.time as f64) - sqrt_discriminant) / 2.0).floor() as u64;
        let max_hold_time = (((self.time as f64) + sqrt_discriminant) / 2.0).ceil() as u64;
        max_hold_time - min_hold_time - 1
    }
}

#[allow(dead_code)]
fn day_6_part_1(data: &str) -> u64 {
    let races = parse_races(data);
    races
        .par_iter()
        .map(|race| race.winning_combo())
        .reduce(|| 1, |a, b| a * b)
}

#[allow(dead_code)]
fn day_6_part_2(data: &str) -> u64 {
    let races = parse_races(data);
    let race = races
        .iter()
        .map(|race| (race.time.to_string(), race.distance.to_string()))
        .reduce(|(acc_time, acc_distance), (time, distance)| {
            (acc_time + &time, acc_distance + &distance)
        })
        .map(|(t, d)| Race {
            time: t.parse::<u64>().unwrap(),
            distance: d.parse::<u64>().unwrap(),
        })
        .unwrap();
    race.winning_combo()
}

fn parse_races(data: &str) -> Vec<Race> {
    let re = Regex::new(r"\d+").unwrap();
    let numbers = re
        .find_iter(data)
        .map(|val| val.as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let time = numbers.iter().take(numbers.len() / 2);
    let distance = numbers.iter().skip(numbers.len() / 2);
    time.zip(distance)
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

#[cfg(test)]
mod test {

    use crate::aoc_2023::day_6::day_6_part_2;

    use super::day_6_part_1;

    #[test]
    fn test_day_6_part_1() {
        let data = include_str!("../../data/aoc_2023/day_6.txt");

        let solution = day_6_part_1(data);
        println!("2023.6.1: {solution}");
    }

    #[test]
    fn test_day_6_part_2() {
        let data = include_str!("../../data/aoc_2023/day_6.txt");

        let solution = day_6_part_2(data);
        println!("2023.6.2: {solution}");
    }
}
