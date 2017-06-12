fn is_same(lhs: &str, rhs: &str) -> bool {
    lhs == rhs
}

fn has_same_size(lhs: &str, rhs: &str) -> bool {
    lhs.len() == rhs.len()
}

fn has_same_elements(lhs: &Vec<char>, rhs: &Vec<char>) -> bool {
    lhs == rhs
}

pub fn matches<'a>(subject: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let lsub = subject.to_lowercase();
    let mut ssub: Vec<char> = lsub.chars().collect();
    ssub.sort();

    candidates.iter()
        .filter(|candidate| {
            let lcan = candidate.to_lowercase();
            let mut scan: Vec<char> = lcan.chars().collect();
            scan.sort();

            has_same_size(&lsub, &lcan) && !is_same(&lsub, &lcan) && has_same_elements(&ssub, &scan)
        })
        .map(|c| *c)
        .collect()
}
