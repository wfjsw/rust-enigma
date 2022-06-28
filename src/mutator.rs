// Mutator: the Mutator trait

pub trait Mutator {
    fn mutate(&mut self, input: char) -> char;
}
