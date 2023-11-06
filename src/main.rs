use std::{io, num};

fn main() {
    println!("Temperature converter");
    loop {
        println!("Choose conversion type: 1. [C->F] or 2. [F->C]");
        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Please, type 1 or 2 to choose a conversion type!");
        let conversion_type: i32 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if conversion_type == 1 {
            println!("chose number 1")
        } else if conversion_type == 2 {
            println!("chose number 2")
        } else {
            println!("Please choose option 1 or 2!")
        }
    }
}
