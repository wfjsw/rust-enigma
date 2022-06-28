// Remapper: the wheel
use super::mutator::Mutator;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Wheel {
    pub mapping: String
}

impl Wheel {
    pub fn mutate_with_offset(&self, input: char, offset: usize, direction: bool) -> char {
        if direction {
            let index = (ALPHABET.find(input).unwrap() + offset) % ALPHABET.len();
            self.mapping.chars().nth(index).unwrap()
        } else {
            let index = (self.mapping.len() + self.mapping.find(input).unwrap() - offset) % self.mapping.len();
            ALPHABET.chars().nth(index).unwrap()
        }

    }
}
