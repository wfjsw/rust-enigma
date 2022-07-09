use indicatif::ProgressBar;
use itertools::Itertools;
use mutator::Mutator;

use crate::wheel::Wheel;

mod mutator;
mod reflector;
mod plugboard;
mod wheel;
mod wheelcage;

mod ioc;

struct Enigma {
    plugboard: plugboard::Plugboard,
    wheelcage: wheelcage::WheelCage,
}

impl Mutator for Enigma {
    fn mutate(&mut self, input: char) -> char {
        let mut output = input;
        output = self.plugboard.mutate(output);
        #[cfg(debug_assertions)]
        println!("Plugboarded: {}", output);
        output = self.wheelcage.mutate(output);
        #[cfg(debug_assertions)]
        println!("Wheelcaged: {}", output);
        output = self.plugboard.mutate(output);
        #[cfg(debug_assertions)]
        println!("Plugboarded: {}", output);
        output
    }
}

impl Enigma {
    fn through(&mut self, input: &str) -> String {
        let mut output = String::new();
        for c in input.chars() {
            output.push(self.mutate(c));
            #[cfg(debug_assertions)]
            println!("----");
        }
        output
    }
}

fn run_enigma(data: &str) -> String{
        let mut enigma = Enigma {
            plugboard: plugboard::Plugboard {
                mapping: vec![
                    // ('A', 'E'),
                    // ('B', 'K'),
                ]
            },
            wheelcage: wheelcage::WheelCage {
                wheels: vec! [
                    (Wheel {
                        mapping: "ZTRXAUIJEFONSHKDCQVWYLMBPG".to_string(),
                    }, 15),
                    (Wheel {
                        mapping: "KHLAPQVCWJSUDRBGOEXYFINZTM".to_string(),
                    }, 21),
                    (Wheel {
                        mapping: "NDUAZGVIMYBEQXWPSRKTCOFJHL".to_string(),
                    }, 6),
                ],
                reflector: reflector::Reflector {
                    mapping: "VRNOHZTPFQDIESJGUCKALWXYMB".to_string(),
                },
            },
        };
    enigma.through(&data)
}

fn main() {
    loop {
        // read line from stdin
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();
        // run through and output
        println!("{}", run_enigma(&input));
        #[cfg(debug_assertions)]
        break;
    }
}

#[test]
fn test_enigma_bruteforce() {
    let example_input = "FORNONNUMERICTYPESTHISCANBECONSIDEREDAMAXIMUMWIDTHIFTHERESULTINGSTRINGISLONGERTHANTHISWIDTHTHENITISTRUNCATEDDOWNTOTHISMANYCHARACTERSANDTHATTRUNCATEDVALUEISEMITTEDWITHPROPERFILLALIGNMENTANDWIDTHIFTHOSEPARAMETERSARESET";
    let example_output = run_enigma(example_input);
    
    // let mut alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    let mut resolution: Vec<(i32, i32, i32, String, String, String, String, f64)> = Vec::new();
    // let mut i = 26*26*26;
    // let bar = ProgressBar::new(4294967296);

    let wheel3 = vec!["KHLAPQVCWJSUDRBGOEXYFINZTM","NDUAZGVIMYBEQXWPSRKTCOFJHL","ZTRXAUIJEFONSHKDCQVWYLMBPG"];
    let wheel3Iter = wheel3.iter();

    for w1 in (0..25) {
        for w2 in (0..25) {
            for w3 in (0..25) {
                for wd3 in wheel3Iter.clone().permutations(3) {

                    // let c1 = w1.iter().collect::<String>();
                    // let c2 = w2.iter().collect::<String>();
                    // let c3 = w3.iter().collect::<String>();

                    let mut enigma = Enigma {
                        plugboard: plugboard::Plugboard {
                            mapping: vec![
                                // ('A', 'E'),
                                // ('B', 'K'),
                            ]
                        },
                        wheelcage: wheelcage::WheelCage {
                            wheels: vec! [
                                (Wheel {
                                    mapping: wd3[0].to_string(),
                                }, w1),
                                (Wheel {
                                    mapping: wd3[1].to_string(),
                                }, w2),
                                (Wheel {
                                    mapping: wd3[2].to_string(),
                                }, w3),
                            ],
                            reflector: reflector::Reflector {
                                mapping: "VRNOHZTPFQDIESJGUCKALWXYMB".to_string(),
                            },
                        },
                    };
                    let resolved_input = enigma.through(&example_output);
                    let ioc = ioc::ioc(&resolved_input);
                    resolution.push((w1,w2,w3,wd3[0].to_string(), wd3[1].to_string(), wd3[2].to_string(),resolved_input, ioc));
                    // if resolved_input == example_input {
                    //     // bar.finish_and_clear();
                    //     println!("Found a match! {:?} ({}, {}, {})", wd3, &w1, &w2, &w3);
                    //     return;
                    // }
                    // bar.inc(1);
                }
            }
        }

    }
    resolution.sort_by(|a, b| b.7.partial_cmp(&a.7).unwrap());
    resolution.truncate(10);

    for (w1, w2, w3, r1, r2, r3, resolved_input, ioc) in resolution {
        println!("{} | {:.4} | {} {} {} | {} {} {}", resolved_input, ioc, w1, w2, w3, r1, r2, r3);
    }
}
