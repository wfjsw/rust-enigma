use super::mutator::Mutator;
use super::wheel::Wheel;
use super::reflector::Reflector;

pub struct WheelCage {
    pub wheels: Vec<(Wheel, i32)>,
    pub reflector: Reflector,
}

impl Mutator for WheelCage {
    fn mutate(&mut self, input: char) -> char {
        let mut output = input;
        for &(ref wheel, offset) in self.wheels.iter() {
            output = wheel.mutate_with_offset(output, offset as usize, true);
            println!("Wheel: {}", output);
        }
        output = self.reflector.mutate(output);
        println!("Reflector: {}", output);
        for &(ref wheel, offset) in self.wheels.iter().rev() {
            output = wheel.mutate_with_offset(output, offset as usize, false);
            println!("Wheel: {}", output);
        }
        self.rotate();
        output
    }
}

impl WheelCage {
    fn rotate(&mut self) {
        for i in (0..self.wheels.len()).rev() {
            self.wheels[i].1 += 1;
            if self.wheels[i].1 == 26 {
                self.wheels[i].1 = 0;
            } else {
                break;
            }
        }
    }
}
