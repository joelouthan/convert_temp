use std::io;

fn main() {
    println!("Please enter a temperature in Fahrenheit:");
    
    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse()
        .expect("Please type a number!");
    
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{} Fahrenheit is {} Celsius", fahrenheit, celsius);

    println!("Please enter a temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin().read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f64 = celsius.trim().parse()
        .expect("Please type a number!");

    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

    println!("{} Celsius is {} Fahrenheit", celsius, fahrenheit);
}
