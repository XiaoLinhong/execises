pub fn check(candidate: &str) -> bool {
    let mut pure_chars = candidate.to_ascii_lowercase().chars()
                .filter(|&c| !" -".contains(c))
                .collect::<Vec<char>>();

    // println!("{:?}", pure_chars);
    let n1 = pure_chars.len();
    pure_chars.sort();
    pure_chars.dedup();
    // println!("{:?}", pure_chars);

    let n2 = pure_chars.len();
    n1 == n2
}
