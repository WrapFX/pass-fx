// Imports
use std::env;
use std::fs;

use rand::Rng;

// Constants
const VALID_CHARS: [&str; 68] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "!", "@", "#", "$", "%", "&",
];

fn main() {
    // Initialize values
    let mut name = String::from("password");
    let mut length = 32;
    let mut password = String::new();

    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    for arg in 0..args.len() {
        // Change name
        if args[arg] == "--name" {
            println!("Attempting output file name change...");
            if args.len() <= arg + 1 {
                println!("Cannot change output file name (next argument is empty)");
                continue;
            }

            name = args[arg + 1].clone();
            println!("Changed output file name to '{}'", args[arg + 1]);
        }

        // Change length
        if args[arg] == "--length" {
            println!("Attempting output length change...");
            if args.len() <= arg + 1 {
                println!("Cannot change output length (next argument is empty)");
                continue;
            }

            match args[arg + 1].as_str().parse::<i32>() {
                Ok(len) => {
                    length = len;
                    println!("Changed output length to {}", len);
                },
                Err(_) => println!("Could not change output length (could not parse argument)"),
            }
        }
    }

    // Generate password
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let char = rng.gen_range(0..VALID_CHARS.len());
        password += VALID_CHARS[char];
    }

    // Output password
    fs::write(format!("./{}.txt", name), password.as_bytes());
    println!("Generated password '{}' with length {}", password, length);
}
