use std::collections::HashMap;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn is_letter(ch: char) -> bool {
    ALPHABET.contains(ch)
}

fn count_letters(s: &str) -> usize {
    s.chars().filter(|&c| is_letter(c)).count()
}

pub fn ioc(s: &str) -> f64 {
    let mut freq: HashMap<char, usize> = HashMap::new();

    for c in ALPHABET.chars() {
        let mut count = 0;
        for ch in s.chars() {
            if ch == c {
                count += 1;
            }
        }
        freq.insert(c, count);
    }

    let mut total = 0;
    for c in ALPHABET.chars() {
        let ni = freq.get(&c).unwrap();
        total += ni * (ni - 1);
    }

    let N = count_letters(s) as f64;

    (total as f64) / (N * (N - 1.0))
}
