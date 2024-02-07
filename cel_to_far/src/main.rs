use std::io;

fn main() {
    println!("Wat is je temperatuur in Celsius?");
    let mut tempcel = String::new();
    
    io::stdin()
        .read_line(&mut tempcel)
        .expect("Failed to read line");

    let tempcel: f32 = tempcel.trim().parse().expect("Please enter a valid number");

    let tempfahr: f32 = (9.0 / 5.0 * tempcel) + 32.0;

    println!("Dit is je temperatuur in Fahrenheit: {}", tempfahr);
}
