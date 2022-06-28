use mutator::Mutator;

use crate::wheel::Wheel;

mod mutator;
mod reflector;
mod plugboard;
mod wheel;
mod wheelcage;

struct Enigma {
    plugboard: plugboard::Plugboard,
    wheelcage: wheelcage::WheelCage,
}

impl Mutator for Enigma {
    fn mutate(&mut self, input: char) -> char {
        let mut output = input;
        output = self.plugboard.mutate(output);
        println!("Plugboarded: {}", output);
        output = self.wheelcage.mutate(output);
        println!("Wheelcaged: {}", output);
        output = self.plugboard.mutate(output);
        println!("Plugboarded: {}", output);
        output
    }
}

impl Enigma {
    fn through(&mut self, input: &str) -> String {
        let mut output = String::new();
        for c in input.chars() {
            output.push(self.mutate(c));
            println!("----");
        }
        output
    }
}

fn main() {
    // read line from stdin
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_uppercase();
    // run through and output
    let mut enigma = Enigma {
        plugboard: plugboard::Plugboard {
            mapping: vec![
                ('A', 'E'),
                ('B', 'K'),
            ]
        },
        wheelcage: wheelcage::WheelCage {
            wheels: vec! [
                (Wheel {
                    mapping: "ZTRXAUIJEFONSHKDCQVWYLMBPG".to_string(),
                }, 0),
                (Wheel {
                    mapping: "KHLAPQVCWJSUDRBGOEXYFINZTM".to_string(),
                }, 0),
                (Wheel {
                    mapping: "NDUAZGVIMYBEQXWPSRKTCOFJHL".to_string(),
                }, 0),
            ],
            reflector: reflector::Reflector {
                mapping: "VRNOHZTPFQDIESJGUCKALWXYMB".to_string(),
            },
        },
    };
    println!("{}", enigma.through(&input));
}
