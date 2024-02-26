fn main() {
    let mut woord = String::from("apple");
    let letter = &woord[0..1];
    let letter_kopie = letter.to_owned();

    match letter_kopie.as_str() {
        "a" | "e" | "i" | "o" | "u" => woord.push_str("hay"),
        _ => {
            woord = woord[1..].to_string();
            woord.push_str(&letter_kopie);
            woord.push_str("ay");
        }
    }
    println!("{}", woord);
}