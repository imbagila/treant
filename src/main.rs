use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://localhost:27017";

    // Create a new client and connect to the server
    let client = Client::with_uri_str(uri).await?;

    // Get a handle on the movies collection
    let database = client.database("mydb");
    let my_coll: Collection<Document> = database.collection("mycoll");

    // Find a movie based on the title value
    let my_movie = my_coll.find_one(doc! { "name": "John Doe" }, None).await?;

    // Print the document
    println!("{:#?}", my_movie);
    Ok(())
}
