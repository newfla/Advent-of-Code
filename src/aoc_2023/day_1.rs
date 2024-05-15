use rayon::{iter::ParallelIterator, str::ParallelString};

#[allow(dead_code)]
fn day_1_part_1(data: &str) -> u32 {
    data.par_lines()
        .map(|line| {
            let chars = line.par_matches(char::is_numeric).collect::<Vec<&str>>();
            (chars[0].to_owned() + chars[chars.len() - 1])
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[allow(dead_code)]
fn day_1_part_2(data: &str) -> u32 {
    data.par_lines()
        .map(|line| {
            let first_digit = find_digit(line, false);
            let second_digit = find_digit(line, true);
            (first_digit + &second_digit).parse::<u32>().unwrap()
        })
        .sum()
}

fn find_digit(data: &str, reverse: bool) -> String {
    let chars: Vec<char> = if reverse {
        data.chars().rev().collect()
    } else {
        data.chars().collect()
    };

    let mut spelled = "".to_owned();
    for char in chars {
        if char.is_numeric() {
            return char.to_string();
        }
        spelled += &char.to_string();
        if let Some(digit) = spelled_number(&spelled, reverse) {
            return digit.to_owned();
        }
    }
    spelled_number(&spelled, reverse).unwrap().to_owned()
}

fn spelled_number(spelled: &str, reverse: bool) -> Option<&'static str> {
    if reverse {
        let reversed = spelled.chars().rev().collect::<String>();
        return match_digit(&reversed);
    }
    match_digit(spelled)
}

fn match_digit(spelled: &str) -> Option<&'static str> {
    if spelled.contains("one") {
        Some("1")
    } else if spelled.contains("two") {
        Some("2")
    } else if spelled.contains("three") {
        Some("3")
    } else if spelled.contains("four") {
        Some("4")
    } else if spelled.contains("five") {
        Some("5")
    } else if spelled.contains("six") {
        Some("6")
    } else if spelled.contains("seven") {
        Some("7")
    } else if spelled.contains("eight") {
        Some("8")
    } else if spelled.contains("nine") {
        Some("9")
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_2023::day_1::day_1_part_2;

    use super::day_1_part_1;

    #[test]
    fn test_day_1_part_1() {
        let data = include_str!("../../data/aoc_2023/day_1.txt");
        let solution = day_1_part_1(data);
        println!("1.1: {solution}");
    }

    #[test]
    fn test_day_1_part_2() {
        let data = include_str!("../../data/aoc_2023/day_1.txt");
        let solution = day_1_part_2(data);
        println!("1.2: {solution}");
    }
}
