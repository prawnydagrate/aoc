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
        /* if let Some(lidx) = update.iter().position(|m| m == l) {
            if let Some(ridx) = update.iter().position(|m| m == r) {
                if lidx >= ridx {
                    return false;
                }
            }
        } */
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

fn build_with_rules(built: &mut Vec<usize>, nums: &[usize], rr: &[(usize, usize)]) -> bool {
    if nums.is_empty() && is_correct(built, rr) {
        return true;
    }
    if built.is_empty() {
        for _ in 0..nums.len() {
            built.push(0);
        }
    }
    for &num in nums.iter() {
        for i in 0..nums.len() {
            if built[i] != 0 {
                continue;
            }
            built[i] = num;
            if !is_correct(built, rr) {
                built[i] = 0;
                continue;
            }
            if build_with_rules(
                built,
                &nums
                    .iter()
                    .filter_map(|&m| if m == num { None } else { Some(m) })
                    .collect::<Vec<_>>(),
                rr,
            ) {
                return true;
            }
            built[i] = 0;
            continue;
        }
    }
    false
}

pub fn solve(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .map(|update| {
            let rr = get_update_relrules(&rules, &update)
                .map(<_>::clone)
                .collect::<Vec<_>>();
            if is_correct(&update, &rr) {
                return 0;
            }
            println!("\nWORKING {update:?}");
            println!("RULES: {rr:?}");
            let mut rebuilt = Vec::new();
            build_with_rules(&mut rebuilt, &update, &rr);
            rebuilt[rebuilt.len() / 2]
        })
        .sum()
}
