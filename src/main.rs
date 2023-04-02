use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    
    println!("Do you want to Convert (1) Degrees to Celcius or (2) Celcius to Degrees?");
    io::stdin().read_line(&mut input).expect("Failed to Read Line");

}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}
fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}