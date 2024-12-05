fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let (rules_str, updates_str) = input
        .trim()
        .split_once("\n\n")
        .expect("input only has one section");
    // parse rules
    for rule_ln in rules_str.lines() {
        let mut rule = Vec::new();
        for n_str in rule_ln.split('|') {
            let n_str = n_str.trim();
            if n_str.is_empty() {
                continue;
            }
            rule.push(n_str.parse().unwrap());
        }
        if !rule.is_empty() {
            rules.push(rule);
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

fn is_correct(update: &[usize], rules: &[Vec<usize>]) -> bool {
    let relevant_rules = rules
        .iter()
        .filter(|v| v.iter().all(|n| update.contains(n)));
    for rule in relevant_rules {
        let mut indices = Vec::new();
        for n in rule {
            indices.push(update.iter().position(|m| m == n).unwrap());
        }
        if !indices.is_sorted() {
            return false;
        }
    }
    true
}

pub fn solve(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .map(|update| {
            if is_correct(&update, &rules) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}
