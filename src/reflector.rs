// Reflector: Connect two characters together
use super::mutator::Mutator;

pub struct Reflector {
    pub mapping: String,
}

impl Mutator for Reflector {
    fn mutate(&mut self, input: char) -> char {
        // find the location of given char in mapping
        let index = self.mapping.find(input).unwrap();
        // return the char at reversal location in mapping
        self.mapping.chars().nth(self.mapping.len() - index - 1).unwrap()
    }
}
