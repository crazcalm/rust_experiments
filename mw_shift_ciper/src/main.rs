use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn encrypt(line: String, shift: i32) -> String {
    let letters_to_numbers: HashMap<char, i32> = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
        ('q', 16),
        ('r', 17),
        ('s', 18),
        ('t', 19),
        ('u', 20),
        ('v', 21),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('z', 25),
    ]);

    let numbers_to_letters: HashMap<i32, char> = HashMap::from([
        (0, 'a'),
        (1, 'b'),
        (2, 'c'),
        (3, 'd'),
        (4, 'e'),
        (5, 'f'),
        (6, 'g'),
        (7, 'h'),
        (8, 'i'),
        (9, 'j'),
        (10, 'k'),
        (11, 'l'),
        (12, 'm'),
        (13, 'n'),
        (14, 'o'),
        (15, 'p'),
        (16, 'q'),
        (17, 'r'),
        (18, 's'),
        (19, 't'),
        (20, 'u'),
        (21, 'v'),
        (22, 'w'),
        (23, 'x'),
        (24, 'y'),
        (25, 'z'),
    ]);

    let mut result = String::new();
    for letter in line.to_lowercase().chars() {
        if let Some(num) = letters_to_numbers.get(&letter) {
            // Trick for modulus (mod(26))
            let new_number = (((num + shift) % 26) + 26) % 26;

            result.push(*numbers_to_letters.get(&new_number).unwrap());
        }
    }

    result
}

fn main() -> io::Result<()> {
    let mut args = env::args();

    if args.len() == 1 {
    } else if args.len() > 2 {
        // Handle args

        // Grab shift value
        let shift: i32 = str::parse(&args.nth(1).unwrap()).expect("shift must be an int");
        for (index, arg) in args.enumerate() {
            if index > 0 {
                println!("");
            }
            let file = File::open(arg)?;
            let buffer = BufReader::new(file);
            for line in buffer.lines() {
                println!("{}", encrypt(line.unwrap(), shift));
            }
        }
    } else {
        // Handle stdin and Pipes
        let shift: i32 = str::parse(&args.nth(1).unwrap()).expect("shift must be an int");
        for line in io::stdin().lock().lines() {
            println!("--> {}", encrypt(line.unwrap(), shift));
        }
    }
    Ok(())
}
