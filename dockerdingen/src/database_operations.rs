use mongodb::{bson::{doc, Document}, Collection};
use std::{error::Error, io};

struct Gebruikers {
    naam: String,
    leeftijd: i32,
}


pub async fn create(coll: &Collection<Document>) -> Result<(), Box<dyn Error>> {
    println!("Wat is de naam van wie je wilt toevoegen?");
    let mut naam = String::new();
    io::stdin()
        .read_line(&mut naam)
        .expect("Failed to read line");

    println!("Wat is de leeftijd van wie je wilt toevoegen?");
    let mut leeftijd = String::new();
    io::stdin()
        .read_line(&mut leeftijd)
        .expect("Failed to read line");

    // Convert leeftijd to an integer
    let leeftijd: i32 = leeftijd.trim().parse().expect("Please enter a valid integer for leeftijd");

    let document = doc! {"naam": naam.trim(), "leeftijd": leeftijd};

    coll.insert_one(document, None).await?;

    println!("Gebruiker succesvol toegevoegd!");

    Ok(())
}

pub async fn read(coll: &Collection<Document>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn update(coll: &Collection<Document>) -> Result<(), Box<dyn Error>> {
    println!("Wat is de naam van de persoon die je wilt bijwerken?");
    let mut naam = String::new();
    io::stdin()
        .read_line(&mut naam)
        .expect("Failed to read line");

    println!("Wat is de nieuwe leeftijd?");
    let mut nieuwe_leeftijd = String::new();
    io::stdin()
        .read_line(&mut nieuwe_leeftijd)
        .expect("Failed to read line");

    // Convert nieuwe_leeftijd to an integer
    let nieuwe_leeftijd: i32 = nieuwe_leeftijd.trim().parse().expect("Please enter a valid integer for leeftijd");

    let filter = doc! {"naam": naam.trim()};
    let update_doc = doc! {"$set": {"leeftijd": nieuwe_leeftijd}};

    let result = coll.update_one(filter, update_doc, None).await?;

    println!("Matched documents: {}", result.matched_count);
    println!("Modified documents: {}", result.modified_count);

    Ok(())
}

pub async fn delete(coll: &Collection<Document>) -> Result<(), Box<dyn Error>> {
    println!("Wat is de naam van de persoon die je wilt verwijderen");
    let mut naam = String::new();
    io::stdin()
        .read_line(&mut naam)
        .expect("Failed to read line");
   
    let filter = doc! {"naam": naam.trim()};
    let result = coll.delete_one(filter, None).await?;
    println!("Deleted documents: {}", result.deleted_count);
    Ok(())
}