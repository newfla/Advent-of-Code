use std::{collections::VecDeque, ops::Range};

use rayon::iter::{
    IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use regex::Regex;

#[derive(Default, Debug)]
struct Mapping {
    source: u32,
    source_range: Range<usize>,
    destination: u32,
}
#[derive(Default, Debug)]
struct GlobalMap {
    seeds: Vec<u32>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl GlobalMap {
    fn lowest_location(&self) -> u32 {
        self.seeds
            .par_iter()
            .map(|seed| self.location(*seed))
            .min()
            .unwrap()
    }

    fn location(&self, seed: u32) -> u32 {
        let soil = Self::find_in_range(seed, &self.seed_to_soil);
        let fertilizer = Self::find_in_range(soil, &self.soil_to_fertilizer);
        let water = Self::find_in_range(fertilizer, &self.fertilizer_to_water);
        let light = Self::find_in_range(water, &self.water_to_light);
        let temperature = Self::find_in_range(light, &self.light_to_temperature);
        let humidity = Self::find_in_range(temperature, &self.temperature_to_humidity);
        Self::find_in_range(humidity, &self.humidity_to_location)
    }

    fn lowest_location_expand(&self) -> u32 {
        let seeds = self.expand_seeds();
        seeds
            .into_iter()
            .map(|range| {
                let mut min = u32::MAX;
                for seed in range {
                    let loc = self.location(seed);
                    if loc < min {
                        min = loc;
                    }
                }
                min
            })
            .min()
            .unwrap()
    }

    fn expand_seeds(&self) -> Vec<Range<u32>> {
        let mut cloned_seeds: VecDeque<u32> = self.seeds.clone().into();
        cloned_seeds.push_front(0);
        let mut end = cloned_seeds.into_iter().step_by(2);
        end.next();

        self.seeds
            .iter()
            .step_by(2)
            .zip(end)
            .par_bridge()
            .map(|(x, y)| (*x..*x + y))
            .collect()
    }

    fn find_in_range(source: u32, mapping: &Vec<Mapping>) -> u32 {
        mapping
            .par_iter()
            .filter(|mapping| mapping.source_range.contains(&(source as usize)))
            .min_by(|x, y| x.destination.cmp(&y.destination))
            .map(|mapping| {
                let step = source - mapping.source;
                mapping.destination + step
            })
            .unwrap_or(source)
    }
}

fn parse_global_map(data: &str) -> GlobalMap {
    let mut map = GlobalMap::default();
    let re = Regex::new(r"\d+").unwrap();
    map.seeds = re
        .find_iter(data.lines().next().unwrap())
        .map(|val| val.as_str().parse::<u32>().unwrap())
        .collect();

    map.seed_to_soil = parse_range(&re, data, find_header(data, "seed-to-soil"));
    map.soil_to_fertilizer = parse_range(&re, data, find_header(data, "soil-to-fertilizer"));
    map.fertilizer_to_water = parse_range(&re, data, find_header(data, "fertilizer-to-water"));
    map.water_to_light = parse_range(&re, data, find_header(data, "water-to-light"));
    map.light_to_temperature = parse_range(&re, data, find_header(data, "light-to-temperature"));
    map.temperature_to_humidity =
        parse_range(&re, data, find_header(data, "temperature-to-humidity"));
    map.humidity_to_location = parse_range(&re, data, find_header(data, "humidity-to-location"));

    map
}

fn find_header(data: &str, hint: &str) -> usize {
    data.lines()
        .enumerate()
        .find_map(|(id, line)| line.contains(hint).then(|| id + 1))
        .unwrap()
}

fn parse_range(regex: &Regex, data: &str, start: usize) -> Vec<Mapping> {
    data.lines()
        .enumerate()
        .filter(|(id, _)| id >= &start)
        .map(|(_, line)| line)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let numbers = regex
                .find_iter(line)
                .map(|val| val.as_str().parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            Mapping {
                source: numbers[1],
                source_range: numbers[1] as usize..numbers[1] as usize + numbers[2] as usize,
                destination: numbers[0],
            }
        })
        .collect()
}

#[allow(dead_code)]
fn day_5_part_1(data: &str) -> u32 {
    parse_global_map(data).lowest_location()
}

#[allow(dead_code)]
fn day_5_part_2(data: &str) -> u32 {
    parse_global_map(data).lowest_location_expand()
}

#[cfg(test)]
mod test {

    use super::{day_5_part_1, day_5_part_2};

    #[test]
    fn test_day_5_part_1() {
        let data = include_str!("../../data/aoc_2023/day_5.txt");

        let solution = day_5_part_1(data);
        println!("2023.5.1: {solution}");
    }

    #[test]
    fn test_day_5_part_2() {
        let data = include_str!("../../data/aoc_2023/day_5.txt");
        let solution = day_5_part_2(data);
        println!("2023.5.2: {solution}");
    }
}
