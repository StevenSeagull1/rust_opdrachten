use std::io;

fn main() {
    //vraag om een getal voor de f waarde
    let mut f = String::new();
    println!("Noem je f getal:");

    io::stdin().read_line(&mut f).expect("Failed to read line");
    //dit checkt of het een getal is
    let f: u32 = f.trim().parse().expect("Dit is geen getal");
    // dit zijn de basis waardes
    let mut eerst = 0;
    let mut nu = 1;
    let mut i = 1;
    //loop net zo vaak om alle waardes te krijgen
    while i < f {
        let volgende = eerst + nu;
        println!("{}", volgende);
        eerst = nu;
        nu = volgende;
        i += 1;
    }
}
