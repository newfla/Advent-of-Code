use std::ops::Range;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Default, Debug)]
struct Mapping {
    source: Range<usize>,
    destination: Range<usize>,
}
#[derive(Default, Debug)]
struct GlobalMap {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl GlobalMap {
    fn lowest_location(&self) -> usize {
        self.seeds
            .par_iter()
            .map(|seed| {
                let soil = Self::find_in_range(*seed, &self.seed_to_soil);
                let fertilizer = Self::find_in_range(soil, &self.soil_to_fertilizer);
                let water = Self::find_in_range(fertilizer, &self.fertilizer_to_water);
                let light = Self::find_in_range(water, &self.water_to_light);
                let temperature = Self::find_in_range(light, &self.light_to_temperature);
                let humidity = Self::find_in_range(temperature, &self.temperature_to_humidity);
                Self::find_in_range(humidity, &self.humidity_to_location)
            })
            .min()
            .unwrap()
    }

    fn expand_seeds(&mut self) {
        let start= self.seeds.iter().step_by(2);
        let mut end = self.seeds.iter();
        end.next();
        self.seeds = start.zip(end.step_by(2)).map(|(x,y)| {println!{"start{x} end{y}"}(*x..*y).collect::<Vec<_>>()}
    ).flatten().collect();
    }

    fn find_in_range(source: usize, mapping: &Vec<Mapping>) -> usize {
        mapping
            .par_iter()
            .filter(|mapping| mapping.source.contains(&source))
            .min_by(|x, y| {
                x.destination
                    .clone()
                    .min()
                    .unwrap()
                    .cmp(&y.destination.clone().min().unwrap())
            })
            .map(|mapping| {
                let step = source - mapping.source.clone().min().unwrap();
                mapping.destination.clone().min().unwrap() + step
            })
            .unwrap_or(source)
    }
}

fn parse_global_map(data: &str, seed_range: bool) -> GlobalMap {
    let mut map = GlobalMap::default();
    let re = Regex::new(r"\d+").unwrap();
    map.seeds = re
        .find_iter(data.lines().next().unwrap())
        .map(|val| val.as_str().parse::<usize>().unwrap())
        .collect();

    if seed_range {
        map.expand_seeds();
    }
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
        .find(|(_, line)| line.contains(hint))
        .unwrap()
        .0
        + 1
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
                .map(|val| val.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Mapping {
                source: numbers[1]..numbers[1] + numbers[2],
                destination: numbers[0]..numbers[0] + numbers[2],
            }
        })
        .collect()
}

#[allow(dead_code)]
fn day_5_part_1(data: &str) -> usize {
    parse_global_map(data, false).lowest_location()
}

#[allow(dead_code)]
fn day_5_part_2(data: &str) -> usize {
    parse_global_map(data, true).lowest_location()

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
