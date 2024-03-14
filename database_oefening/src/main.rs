use mongodb::{Client, options::ClientOptions, bson::{doc, Document}, Collection};

struct Accounts {
    _id: i32,
    account_id: i32,
    limit: i32,
    products: Vec<&'static str>,
}

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // verbindings link
    let client_options = ClientOptions::parse("/").await?;
    // de connectie met de server
    
    let client = Client::with_options(client_options)?;

    // de juiste database pakken
    let db = client.database("sample_analytics");

    // en hier de juiste tabel
    let my_coll: Collection<Document> = db.collection("accounts");

    let docs = vec![
        Accounts {
            _id: 8,
            account_id: 125432,
            limit: 8903,
            products: vec!["switch", "ps4"],
        },
        Accounts {
            _id: 9,
            account_id: 765272,
            limit: 3000,
            products: vec!["wii", "ps1"],
        },
        Accounts {
            _id: 10,
            account_id: 334254,
            limit: 1000,
            products: vec!["3ds", "xbox"],
        }
    ];

    let bson_docs: Vec<Document> = docs.iter().map(|account| {
        doc! {
            "_id": account._id,
            "account_id": account.account_id,
            "limit": account.limit,
            "products": account.products.clone(),
        }
    }).collect();

    let insert_many_result = my_coll.insert_many(bson_docs, None).await?;
    for (_key, value) in &insert_many_result.inserted_ids {
        println!("deze id's zijn toegevoegd {:?}", value);
    }

    Ok(())
}