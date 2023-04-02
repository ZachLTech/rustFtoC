use std::io;
use std::io::Read;

fn main() {
    let mut answer = String::new();
    
    println!("Do you want to Convert (1) Fahrenheit to Celcius or (2) Celcius to Fahrenheit?: ");
    io::stdin().read_line(&mut answer).expect("Failed to Read Line");

    let answer = answer.trim().parse::<i32>().unwrap();
    if answer == 1{
        let mut temp = String::new();
        println!("What's the temperature you want to convert?: ");
        io::stdin().read_line(&mut temp).expect("Failed to Read Line");
        let mut temp = temp.trim().parse::<f64>().unwrap();
        fahrenheit_to_celcius(temp);
    }
    else if answer == 2 {
        let mut temp = String::new();
        println!("What's the temperature you want to convert?: ");
        io::stdin().read_line(&mut temp).expect("Failed to Read Line");
        let mut temp = temp.trim().parse::<f64>().unwrap();
        celcius_to_fahrenheit(temp);
    }
    else {
        println!("That's Not an option.");
    }
}

fn celcius_to_fahrenheit(mut celcius: f64) {
    celcius = celcius * 1.8 + 32.0;
    println!("Your converted temp is: {celcius}");
}
fn fahrenheit_to_celcius(mut fahrenheit: f64) {
    fahrenheit = (fahrenheit - 32.0) / 1.8;
    println!("Your converted temp is: {fahrenheit}");
}