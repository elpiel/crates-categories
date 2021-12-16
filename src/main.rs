use crates_categories::fetch_all_categories;
use crates_categories::fetch_full_details;
use crates_io_api::{SyncClient};
use std::fs;



fn main() {
    
    //Have a list of categories (configuration) to fetch from crates.io
    let list_of_categories: [&str; 3] = ["algorithms", "asynchronous", "games"];

    //create a SyncClient
    let client = SyncClient::new(
        "IPCA - Rust",
        std::time::Duration::from_millis(1), //Resquests to do per second.
    )
    .expect("Error making the client");

    let categories_crates = fetch_all_categories(&client, &list_of_categories)
        .expect("Should fetch all crates of every category and return them");

    // Just a test to print the crates that we have fetched to the categories_Crates hashmap (AND ITS WORKING FINE) you can ignore this
    for (categorie, krate) in categories_crates.clone() {
        for x in krate {
            println!("Category: {} Crate {}", categorie ,x.name)
        }

    }
    
    //Fetch each and every crates for each and every category to get the full details.   
    let category_crates_with_full_details = fetch_full_details(&client, &categories_crates)
        .expect("Should collect all the information of every crate");


    // Test to print the new Hash map to check if the full crate are correct 
    for (categorie ,krate ) in category_crates_with_full_details.iter() {
        for k in krate.iter() {
            println!("Category: {} Full Crate Name: {}",categorie , k.name);
        }
    }

    // Serialize the full crates, to .json files for each categorie. 
    for (categorie, krate   ) in category_crates_with_full_details.iter() {
        let vec_full_crates_to_json = serde_json::to_string_pretty(krate)
        .expect("Error serializing");
        
        fs::write(format!("{}.json", categorie.to_string()), vec_full_crates_to_json)
        .expect("Failed to write the file");

    }
    
}