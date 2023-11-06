use std::io;

fn main() {
    println!("========Temperature converter========");
    loop {
        println!("Choose conversion type: [1] °C->°F [2] °F->°C [3] Quit and press [enter]");
        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Please, type 1 or 2 to choose a conversion type!");
        let conversion_type: i32 = match conversion_type.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mut input_temperature = String::new();
        if conversion_type == 1 {
            println!("You chose to convert Celsius to Fahrenheit.");
            println!("Please input the temperature in *Celsius* and press [enter]");
            io::stdin()
                .read_line(&mut input_temperature)
                .expect("Error reading input!");
            let temp_in_float: f64 = match input_temperature.trim().parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            let converted_temperature = c_to_f(temp_in_float);
            println!("The converted temperature is: {converted_temperature}°F.");
            println!("=====================================")
        } else if conversion_type == 2 {
            println!("You chose to convert Fahrenheit to Celsius.");
            println!("Please input the temperature in *Fahrenheit* and press [enter]");
            io::stdin()
                .read_line(&mut input_temperature)
                .expect("Error reading input!");
            let temp_in_float: f64 = match input_temperature.trim().parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            let converted_temperature = f_to_c(temp_in_float);
            println!("The converted temperature is: {converted_temperature}°C");
            println!("=====================================")
        } else if conversion_type == 3 {
            break;
        } else {
            println!("ERROR: PLEASE CHOOSE OPTIONS [1], [2] or [3]!")
        }
    }
}

fn c_to_f(temperature: f64) -> f64 {
    temperature * 9.0 / 5.0 + 32.0
}

fn f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}
