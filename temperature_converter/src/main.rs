use std::io;

const CELSIUS_FAHR_FACTOR: f64 = 1.8;
const TEMP_DELTA: u32 = 32;

fn main() {
    println!("Input the desired operation: 1-> C to F, 2-> F to C");

    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation: u32 = operation.trim().parse().expect("Conversion failed");

    println!("Input temperatur");

    let mut input_temp = String::new();
    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    let input_temp: u32 = input_temp.trim().parse().expect("Conversion failed");

    if operation == 1 {
        let result = celsius_to_fahrenheit(input_temp);
        println!("Input celsius: {} in fahrenheit: {}", input_temp, result);
    } else if operation == 2 {
        let result = fahrenheit_to_celsius(input_temp);
        println!("Input fahrenheit: {} in celsius: {}", input_temp, result);
    } else {
        println!("Invalid operation");
    }
}

fn celsius_to_fahrenheit(celcius_temp: u32) -> f64 {
    f64::from(celcius_temp) * CELSIUS_FAHR_FACTOR + f64::from(TEMP_DELTA)
}

fn fahrenheit_to_celsius(fahrenheit_temp: u32) -> f64 {
    f64::from(fahrenheit_temp - TEMP_DELTA) * CELSIUS_FAHR_FACTOR
}
