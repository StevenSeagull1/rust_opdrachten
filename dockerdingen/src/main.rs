use mongodb::{
    bson::{doc, Document, to_document},
    options::ClientOptions,
    Collection,
    Client,
};
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::io;

// Define Gebruikers struct
#[derive(Debug, Serialize, Deserialize)]
struct Gebruikers {
    naam: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the MongoDB server
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let my_coll: Collection<Document> = client.database("app").collection("gebruikers");

    println!("Connected to MongoDB!");
    let mut input = String::new();

 let result = my_coll.find_one(
        doc! { "naam": "steven" },
        None
    ).await?;
    println!("{:#?}", result);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Create a Gebruikers instance
    let doc = Gebruikers {
        naam: input.trim().to_string(),
    };

    // Print the document (for debugging purposes)
    println!("{:?}", doc);

    // Convert Gebruikers struct to BSON document
    let bson_doc = to_document(&doc)?;

    // Insert the document into the collection

    // match my_coll.insert_one(bson_doc, None).await {
    //     Ok(_) => println!("Document inserted successfully!"),
    //     Err(e) => eprintln!("Failed to insert document: {}", e),
    // }

    let filter = doc! { "naam": "tim" };
    // let update = doc! { "$set": doc! {"leeftijd": "22"} };

    // let res = my_coll.update_one(filter, update, None).await?;
    // println!("Updated documents: {}", res.modified_count);

    let result = my_coll.delete_one(filter, None).await?;
    println!("Deleted documents: {}", result.deleted_count);
    Ok(())
}
 