use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("invalid spring: {ch:?}"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Record {
    pub springs: Vec<Spring>,
    pub pattern: Vec<usize>,
}

impl Record {
    pub fn new(springs: Vec<Spring>, pattern: Vec<usize>) -> Self {
        Self { springs, pattern }
    }

    pub fn parse(line: &str) -> Self {
        Self {
            springs: line
                .split_whitespace()
                .next()
                .unwrap()
                .trim()
                .chars()
                .map(|ch| ch.into())
                .collect(),
            pattern: line
                .split_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    pub fn parse_many(lines: &str) -> Vec<Self> {
        lines.lines().map(|line| Self::parse(line)).collect()
    }

    pub fn expanded(&self) -> Self {
        Self::new(
            self.springs
                .iter()
                .cloned()
                .chain(std::iter::once(Spring::Unknown))
                .cycle()
                .take(self.springs.len() * 5 + 4)
                .collect(),
            self.pattern
                .iter()
                .cloned()
                .cycle()
                .take(self.pattern.len() * 5)
                .collect(),
        )
    }

    pub fn expand(&mut self) {
        *self = self.expanded();
    }
}

pub fn number_of_solutions(memo: &mut HashMap<Record, usize>, record: &Record) -> usize {
    // If we've already seen this one, just return
    if let Some(&n) = memo.get(record) {
        return n;
    }

    if record.pattern.is_empty() {
        // We have a solution if there are no damaged springs in the list anymore.
        let n = if record
            .springs
            .iter()
            .all(|&spring| spring != Spring::Damaged)
        {
            1
        } else {
            0
        };
        memo.insert(record.clone(), n);
        return n;
    }

    // We have some number of groups left, so we need to make sure that we have enough springs for
    // them.
    if record.springs.len() < record.pattern.iter().sum::<usize>() + record.pattern.len() - 1 {
        // Example of why this works:
        // If we have pattern = [3, 4, 2], we must have AT LEAST
        // ???.????.??
        // which is 3 + 1 + 4 + 1 + 2
        // whichi s 3 + 4 + 2 + 2
        // which is pattern.sum() + pattern.len() - 1
        memo.insert(record.clone(), 0);
        return 0;
    }

    // If the first spring is operational, just skip
    if record.springs[0] == Spring::Operational {
        let n = number_of_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.pattern.clone()),
        );
        memo.insert(record.clone(), n);
        return n;
    }

    // Now we know that we are at the beginning of a possible position for the current group.
    // Therefore we can check if it is possible, and if so, we can see how many possible solutions
    // we'd have.
    let mut n = 0;
    let separator_idx = record.pattern[0]; // The current group / the index of the spring right after
                                           // this group (which must be an operational 'separator')
    let after = (separator_idx + 1).min(record.springs.len()); // The index of the spring right after
                                                               // the operational 'separator' (the
                                                               // start of the next group)
    let all_non_op = record.springs[..separator_idx]
        .iter()
        .all(|&spring| spring != Spring::Operational); // All springs in this group must be
                                                       // non-operational (? or #) for this group
                                                       // to be possible at this position
    if all_non_op
        && (record.springs.len() <= separator_idx
            || (
                record.springs.len() > separator_idx
                    && record.springs[separator_idx] != Spring::Damaged
                // This ensures that the
                // separator can act as one, by
                // being . or ?
            ))
    {
        // This group is possible at this position, so we move on to the next group
        n = number_of_solutions(
            memo,
            &Record::new(
                record.springs[after..].to_vec(),
                record.pattern[1..].to_vec(),
            ),
        );
    }

    // We could also choose not to use it if the current position is an unknown spring, so we
    // include these possibilities as well.
    if record.springs[0] == Spring::Unknown {
        n += number_of_solutions(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.pattern.clone()),
        );
    }

    // Now we have the number of solutions for this record, so we memoize and return
    memo.insert(record.clone(), n);
    n
}

pub fn solve(input: &str, part: usize) -> usize {
    let mut memo = HashMap::new();
    Record::parse_many(input)
        .into_iter()
        .map(|mut record| {
            match part {
                1 => (),
                2 => record.expand(),
                _ => panic!("invalid part: {part:?}"),
            }
            number_of_solutions(&mut memo, &record)
        })
        .sum()
}
