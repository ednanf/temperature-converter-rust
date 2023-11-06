use std::io;

fn main() {
    println!("Temperature converter");
    println!("Choose conversion type: 1. [C->F] or 2. [F->C]");
    let mut conversion_type = String::new();

    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Please, type 1 or 2 to choose a conversion type!");

    println!("The number is {conversion_type}");
}
