use mongodb::{
    bson::{Document},
    options::ClientOptions,
    Collection,
    Client,
};
mod database_operations;

use database_operations::{create, read, update, delete};
use std::error::Error;
use std::io;

enum Commando {
    Stop,
    Toevoegen,
    Update,
    Ophalen,
    Verwijderen,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // verbinding met mongo
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let my_coll: Collection<Document> = client.database("app").collection("gebruikers");

    println!("Connected to MongoDB!");

    loop {
        println!("Voer een commando in ('toevoegen', 'update', 'ophalen', 'verwijderen', 'stop'): ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // Verander de input naar lowercase.
        // en match de input met een commando.
        // commando wordt in een variabele gezet
        // als het niet overeenkomt wordt er een foutmelding gegeven
        let commando = match input.trim().to_lowercase().as_str() {
            "update" => Commando::Update,
            "toevoegen" => Commando::Toevoegen,
            "ophalen" => Commando::Ophalen,
            "verwijderen" => Commando::Verwijderen,
            "stop" => Commando::Stop,
            _ => {
                println!("Ongeldig commando, probeer opnieuw.");
                continue;
            }
        };
        match commando {
            Commando::Toevoegen => create(&my_coll).await?,
            Commando::Ophalen => read(&my_coll).await?,
            Commando::Update => update(&my_coll).await?,
            Commando::Verwijderen => delete(&my_coll).await?,
            Commando::Stop => break,
        }
    }

    Ok(())
}