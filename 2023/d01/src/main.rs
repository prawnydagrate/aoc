use std::collections::HashMap;

fn get_calibration_value(line: &str) -> usize {
    let spelled_digits: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let (mut first_digit, mut last_digit) = ('0', '0');
    'main: for (idx, ch) in line.chars().enumerate() {
        if ch.is_ascii_digit() {
            first_digit = ch;
            break;
        }
        for word in spelled_digits.keys() {
            if line
                .chars()
                .skip(idx)
                .take(word.len())
                .collect::<String>()
                .contains(word)
            {
                first_digit = *spelled_digits.get(word).unwrap();
                break 'main;
            }
        }
    }
    'main: for (idx, ch) in line.chars().rev().enumerate() {
        if ch.is_ascii_digit() {
            last_digit = ch;
            break;
        }
        for word in spelled_digits.keys() {
            if line
                .chars()
                .rev()
                .skip(idx)
                .take(word.len())
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
                .contains(word)
            {
                last_digit = *spelled_digits.get(word).unwrap();
                break 'main;
            }
        }
    }
    format!("{first_digit}{last_digit}").parse().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::get_calibration_value;

    #[test]
    fn example() {
        let lines = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen\
";
        let mut total = 0;
        for line in lines.lines() {
            total += get_calibration_value(line);
        }
        assert_eq!(total, 281);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut total = 0;
    for line in input.lines() {
        total += get_calibration_value(line);
    }
    println!("Got the total {total}");
}
