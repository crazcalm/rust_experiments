use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args = env::args();

    if args.len() > 1 {
        // Handle args
        for (index, arg) in args.skip(1).enumerate() {
            if index > 0 {
                println!("");
            }
            let file = File::open(arg)?;
            let buffer = BufReader::new(file);
            for line in buffer.lines() {
                println!("{}", line.unwrap());
            }
        }
    } else {
        // Handle stdin and Pipes
        for line in io::stdin().lock().lines() {
            println!("--> {}", line.unwrap());
        }
    }
    Ok(())
}
