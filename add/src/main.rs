use std::collections::HashMap;
use std::io;

enum Commando {
    Stop,
    Toevoegen,
    Show,
}

fn main() {
    let mut lijst = HashMap::new();
    //loopt net zolang door als dat de gebruiker wilt
    loop {
        println!("Voer een commando in ('stop', 'toevoegen', 'show'): ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        //verander de input naar lowercase.
        // en match de input met een commando.
        // commando wordt in een variabele gezet
        //als het niet overeenkomt wordt er een foutmelding gegeven
        let commando = match input.trim().to_lowercase().as_str() {
            "stop" => Commando::Stop,
            "toevoegen" => Commando::Toevoegen,
            "show" => Commando::Show,
            _ => {
                println!("Ongeldig commando, probeer opnieuw.");
                continue;
            }
        };
        // commando dat hierboven in de variabele word gedaan word hier gematch
        match commando {
            Commando::Stop => break,
            Commando::Show => show(&lijst),
            Commando::Toevoegen => toevoegen(&mut lijst),
        }
    }
}

fn toevoegen(lijst: &mut HashMap<String, String>) {
    let mut naam = String::new();
    loop {
        println!("Wie wil je toevoegen?");
        io::stdin()
            .read_line(&mut naam)
            .expect("Failed to read line");

        if !naam.trim().is_empty() {
            break;
        } else {
            println!("Naam mag niet leeg zijn.");
        }
    }

    let mut afdeling = String::new();
    loop {
        println!("Voer de afdeling in:");
        io::stdin()
            .read_line(&mut afdeling)
            .expect("Failed to read line");

        if !afdeling.trim().is_empty() {
            break;
        } else {
            println!("Afdeling mag niet leeg zijn.");
        }
    }
    //de trim functie haalt alle witruimtes weg
    // voor de rest worden de gegevens hier in een hashmap gedaan
    lijst.insert(naam.trim().to_string(), afdeling.trim().to_string());
    println!("{} toegevoegd aan de lijst.", naam.trim());
}

fn show(lijst: &HashMap<String, String>) {
    println!("Lijst van personen met hun afdeling:");
    // sorteer de hashmap
    let mut sorted_list: Vec<_> = lijst.iter().collect();
    sorted_list.sort_by_key(|(naam, _)| naam.clone());
    //hier wordt er geloopt door de hashmap en laat de code alles zien
    for (naam, afdeling) in sorted_list {
        println!("Naam: {}, Afdeling: {}", naam, afdeling);
    }
}
