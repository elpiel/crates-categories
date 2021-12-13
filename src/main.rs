use crates_categories::fetch_all_categories;
use crates_categories::fetch_full_details;
//use crates_categories::fetch_all_categories;
use crates_io_api::{SyncClient};
//use std::collections::HashMap;


fn main() {
    //Have a list of categories (configuration) to fetch from crates.io
    let list_of_categories: [&str; 3] = ["algorithms", "asynchronous", "games"];

    //create a SyncClient
    let client = SyncClient::new(
        "IPCA - Rust",
        std::time::Duration::from_millis(1000), //Resquests to do per second.
    )
    .expect("Error making the client");

    let categories_crates = fetch_all_categories(&client, &list_of_categories)
        .expect("Should fetch all crates of every category and return them");

    // Just a test to print the crates that we have fetched to the categories_Crates hashmap (AND ITS WORKING FINE) you can ignore this
    for (categorie, krate) in categories_crates {
        for x in krate {
            println!("Category: {} Crate {}", categorie ,x.name)
        }

    }
    
    // TODO: fetch each and every crates for each and every category to get the full details.
    // the `Crate` in `categories_crates` does not contain all the information you need.
    // let category_crates_with_full_details = my_function(&client, &categories_crates).expect("Should collect all the information of every crate");
    let category_crates_with_full_details = fetch_full_details(&client, &categories_crates)
        .expect("Should collect all the information of every crate");

    

    

    
    
    
    
    
    // TODO: Save results to file?!
}