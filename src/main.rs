mod parsetime;

use std::env;

fn usage(name: String) {
    println!("Usage: {} [timespec]", name)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let arg = args[1].clone();
        let result = parsetime::parsetime(&*arg);
        println!("{}", result.asctime());
    } else {
        usage(args[0].clone());
    }
}

