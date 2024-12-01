use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_nums: Vec<usize>,
    nums_in_possession: Vec<usize>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let card_id_main: Vec<_> = line.split(": ").collect();
        let (card_id_str, main_str) = (card_id_main[0], card_id_main[1]);
        let id = card_id_str.as_bytes()[5..]
            .iter()
            .map(|&b| b as char)
            .collect::<String>()
            .trim()
            .parse()
            .unwrap();
        let winning_have: Vec<_> = main_str.split(" | ").collect();
        let (winning_str, have_str) = (winning_have[0], winning_have[1]);
        let (winning_nums, nums_in_possession) = (get_numbers(winning_str), get_numbers(have_str));
        Self {
            id,
            winning_nums,
            nums_in_possession,
        }
    }

    fn get_nwinning(&self) -> usize {
        let mut winning = Vec::new();
        for num in self.nums_in_possession.iter() {
            if self.winning_nums.contains(num) {
                winning.push(*num);
            }
        }
        winning.len()
    }
}

fn get_numbers(line: &str) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut idx = 0;
    let bytes = line.as_bytes();
    let line_len = line.len();
    while idx < line_len {
        let ch = bytes[idx] as char;
        if !ch.is_ascii_digit() {
            idx += 1;
            continue;
        }
        let mut digits = vec![ch];
        loop {
            idx += 1;
            if idx == line_len {
                break;
            }
            let ch = bytes[idx] as char;
            if !ch.is_ascii_digit() {
                break;
            }
            digits.push(bytes[idx] as char);
        }
        if !digits.is_empty() {
            numbers.push(digits.iter().collect::<String>().trim().parse().unwrap());
        }
        idx += 1;
    }
    numbers
}

// fn solve(input: &str) -> usize {
//     let mut orig_cards = HashMap::new();
//     let mut max_id = 0;
//     for line in input.lines() {
//         let card = Card::from_line(line);
//         orig_cards.insert(card.id, card);
//         max_id += 1;
//     }
//     let mut nprocessed = 0;
//     let mut unprocessed: Vec<_> = orig_cards.keys().map(|&id| id).collect();
//     while !unprocessed.is_empty() {
//         println!("{unprocessed:?}");
//         let mut remove_indices = Vec::new();
//         let mut push_ids = Vec::new();
//         for (idx, &id) in unprocessed.iter().enumerate() {
//             nprocessed += 1;
//             remove_indices.push(idx);
//             let card = orig_cards.get(&id).unwrap();
//             let nwinning = card.get_nwinning();
//             if nwinning == 0 {
//                 continue;
//             }
//             let mut lower = id + 1;
//             lower = if lower <= max_id { lower } else { max_id };
//             let mut upper = id + nwinning;
//             upper = if upper <= max_id { upper } else { max_id };
//             push_ids.extend(lower..=upper);
//         }
//         unprocessed = unprocessed
//             .into_iter()
//             .enumerate()
//             .filter(|(idx, _)| !remove_indices.contains(&idx))
//             .map(|(_, id)| id)
//             .collect();
//         unprocessed.append(&mut push_ids);
//     }
//     nprocessed
// }

fn solve(input: &str) -> usize {
    let mut orig_cards = HashMap::new();
    let mut max_id = 0;
    for line in input.lines() {
        let card = Card::from_line(line);
        orig_cards.insert(card.id, card);
        max_id += 1;
    }
    get_total_cards(
        orig_cards.keys().map(|&id| id).collect(),
        &orig_cards,
        max_id,
    )
}

fn get_total_cards(ids: Vec<usize>, orig_cards: &HashMap<usize, Card>, max_id: usize) -> usize {
    let mut total = 0;
    for id in ids {
        total += 1;
        let nwinning = orig_cards.get(&id).unwrap().get_nwinning();
        if nwinning == 0 {
            continue;
        }
        let mut lower = id + 1;
        lower = if lower <= max_id { lower } else { max_id };
        let mut upper = id + nwinning;
        upper = if upper <= max_id { upper } else { max_id };
        total += get_total_cards((lower..=upper).collect(), orig_cards, max_id);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve(input), 30);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the sum: {}", solve(input));
}
