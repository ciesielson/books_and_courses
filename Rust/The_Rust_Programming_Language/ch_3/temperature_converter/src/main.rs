use std::io;

fn main() {
    loop {
        println!("This is temperature converter.");
        println!("Please select conversion type.");
        println!("1. Celsius to Fahrenheit.");
        println!("2. Fahrenheit to Celsius.");

        let mut conversion_type = String::new();

        io::stdin().read_line(&mut conversion_type).expect("failed to read line");

        let conversion_type = conversion_type.trim();
        let conversion_type = match conversion_type {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };

        println!("Please input the temperature");
        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature).expect("failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a valid temperature");
                continue;
            }
        };

        let converted_temperature = if conversion_type == 1 {
            temperature * 1.8 + 32.
        } else {
            (temperature - 32.) / 1.8
        };

        println!("Converted temperature is {}", converted_temperature)
    }
}
