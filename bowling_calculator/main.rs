// from standard, use environment args and type Args
use std::env::{args, Args};


fn main() {
    // retrieve args from cli
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let second = args.nth(0).unwrap();
    let third = args.nth(0).unwrap();
    let fourth = args.nth(0).unwrap();
    let fifth = args.nth(0).unwrap();
    /*
    let sixth = args.nth(0).unwrap();
    let seventh = args.nth(0).unwrap();
    let eighth = args.nth(0).unwrap();
    let nineth = args.nth(0).unwrap();
    let tenth = args.nth(0).unwrap();
    let eleventh = args.nth(0).unwrap();
    let twelfth = args.nth(0).unwrap();
    let thirteenth = args.nth(0).unwrap();
    */
    println!("{} {} {} {} {}", first, second, third, fourth, fifth);
}

// fn calculate_score