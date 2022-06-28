// Plugboard: Remaps pairs of key
use super::mutator::Mutator;

pub struct Plugboard {
    pub mapping: Vec<(char, char)>,
}

impl Mutator for Plugboard {
    fn mutate(&mut self, input: char) -> char {
        // enumerate through mapping, check if input character is in either side of mapping pair
        for (a, b) in self.mapping.iter() {
            if input == *a || input == *b {
                // if so, return the other side of the mapping pair
                return if input == *a { *b } else { *a };
            }
        }
        // if not, return the input character
        input
    }
}
