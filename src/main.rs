use crates_categories::fetch_all_categories;
//use crates_categories::fetch_all_categories;
use crates_io_api::{Error, ListOptions, Sort, SyncClient};
use std::collections::HashMap;

fn list_top_dependencies() -> Result<(), Error> {
    // Instantiate the client.
    let client = SyncClient::new(
        "IPCA - Rust",
        std::time::Duration::from_millis(1000), //Resquests to do per second.
    )?;

    // Retrieve summary data.
    let summary = client.summary()?;
    for c in summary.most_downloaded {
        println!("{}:", c.id);
    }
    Ok(())
}

fn main() {
    //Have a list of categories (configuration) to fetch from crates.io
    let list_of_categories: [&str; 2] = ["algorithms", "asynchronous"];

    //create a SyncClient
    let client = SyncClient::new(
        "IPCA - Rust",
        std::time::Duration::from_millis(1000), //Resquests to do per second.
    )
    .expect("Error making the client");

    let results = fetch_all_categories(&client, &list_of_categories)
        .expect("Should fetch all crates of every category and return them");

    // TODO: Save results to file?!
}
