fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let (rules_str, updates_str) = input
        .trim()
        .split_once("\n\n")
        .expect("input only has one section");
    // parse rules
    for rule_ln in rules_str.lines() {
        if let Some((l, r)) = rule_ln.split_once('|') {
            rules.push((l.trim().parse().unwrap(), r.trim().parse().unwrap()));
        }
    }
    // parse updates
    for update_ln in updates_str.lines() {
        let mut update = Vec::new();
        for n_str in update_ln.split(',') {
            let n_str = n_str.trim();
            if n_str.is_empty() {
                continue;
            }
            update.push(n_str.parse().unwrap());
        }
        if !update.is_empty() {
            updates.push(update);
        }
    }
    (rules, updates)
}

fn get_update_relrules<'a>(
    rules: &'a [(usize, usize)],
    update: &'a [usize],
) -> impl Iterator<Item = &'a (usize, usize)> {
    rules
        .iter()
        .filter(|(l, r)| update.contains(l) && update.contains(r))
}

fn is_correct(update: &[usize], relevant_rules: &[(usize, usize)]) -> bool {
    for (left, right) in relevant_rules {
        let lpos = update.iter().position(|m| m == left);
        let rpos = update.iter().position(|m| m == right);
        if let Some(l) = lpos {
            if matches!(update.iter().skip(l + 1).next(), None) {
                return false;
            }
        }
        if let Some(r) = rpos {
            if matches!(update.iter().take(r).rev().next(), None) {
                return false;
            }
        }
        if let Some(l) = lpos {
            if let Some(r) = rpos {
                if l >= r {
                    return false;
                }
            }
        }
    }
    true
}

pub fn solve(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .map(|mut update| {
            let rr = get_update_relrules(&rules, &update)
                .map(<_>::clone)
                .collect::<Vec<_>>();
            if is_correct(&update, &rr) {
                return 0;
            }
            println!("WORKING {update:?}");
            while !is_correct(&update, &rr) {
                for ij in (0..update.len()).collect::<Vec<_>>().windows(2) {
                    let (i, j) = (ij[0], ij[1]);
                    let (a, b) = (update[i], update[j]);
                    if rr.iter().find(|(x, y)| x == &b && y == &a).is_some() {
                        (update[i], update[j]) = (b, a);
                    }
                }
            }
            update[update.len() / 2]
        })
        .sum()
}
