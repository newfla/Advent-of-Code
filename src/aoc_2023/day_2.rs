use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

#[derive(Default)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    fn filter(self, condition: Subset) -> Option<Self> {
        match self.subsets.par_iter().find_any(|set| {
            set.red > condition.red || set.green > condition.green || set.blue > condition.blue
        }) {
            Some(_) => None,
            None => Some(self),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Subset {
    red: u8,
    green: u8,
    blue: u8,
}

impl Subset {
    fn power(self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

#[allow(dead_code)]
fn day_2_part_1(data: &str, condition: Subset) -> u32 {
    data.par_lines()
        .filter_map(|line| ugly_game_parser(line).filter(condition))
        .fold_with(0, |acc, elem| acc + elem.id)
        .sum()
}

#[allow(dead_code)]
fn day_2_part_2(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let mut max_set = Subset::default();
            for set in ugly_game_parser(line).subsets {
                if set.red > max_set.red {
                    max_set.red = set.red
                }
                if set.green > max_set.green {
                    max_set.green = set.green
                }
                if set.blue > max_set.blue {
                    max_set.blue = set.blue
                }
            }
            max_set.power()
        })
        .sum()
}

fn ugly_game_parser(data: &str) -> Game {
    let mut game = Game::default();
    let (id_str, subsets_str) = data.split_once(':').unwrap();
    game.id = id_str[5..id_str.len()].parse::<u32>().unwrap();
    for set in subsets_str.split(';') {
        game.subsets.push(ugly_subset_parser(set));
    }
    game
}

fn ugly_subset_parser(data: &str) -> Subset {
    let mut subset = Subset::default();

    for cube in data.split(',') {
        let count = cube
            .par_matches(char::is_numeric)
            .collect::<String>()
            .parse::<u8>()
            .unwrap();
        if cube.contains("red") {
            subset.red += count;
        } else if cube.contains("green") {
            subset.green += count;
        } else if cube.contains("blue") {
            subset.blue += count;
        }
    }

    subset
}

#[cfg(test)]
mod test {

    use crate::aoc_2023::day_2::Subset;

    use super::{day_2_part_1, day_2_part_2};

    #[test]
    fn test_day_2_part_1() {
        let data = include_str!("../../data/aoc_2023/day_2.txt");
        let condition = Subset {
            red: 12,
            green: 13,
            blue: 14,
        };

        let solution = day_2_part_1(data, condition);
        println!("2023.2.1: {solution}");
    }

    #[test]
    fn test_day_2_part_2() {
        let data = include_str!("../../data/aoc_2023/day_2.txt");
        let solution = day_2_part_2(data);
        println!("2023.2.2: {solution}");
    }
}
