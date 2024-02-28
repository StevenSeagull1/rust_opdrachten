use std::io;

fn main() {
    println!("wat is de naam van wie je wilt toevoegen?");
    let mut naam = String::new();
    io::stdin()
        .read_line(&mut naam)
        .expect("Failed to read line");
    println!("wat is de leeftijd van wie je wilt toevoegen?");
    let mut leeftijd = String::new();
    io::stdin()
        .read_line(&mut leeftijd)
        .expect("Failed to read line");
}
