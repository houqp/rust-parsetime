mod parsetime;

use std::env;

fn usage(name: String) {
    println!("Usage: {} [timespec]", name)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let result = parsetime::parsetime(args[1].clone());
        println!("{}", result.asctime());
    } else {
        usage(args[0].clone());
    }
}

