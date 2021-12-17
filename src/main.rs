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
        std::time::Duration::from_millis(1000), //Resquests to do per second.
    )
    .expect("Error making the client");

    //Fetch all crates from the given list_of_categories to an HashMap - as colection to a vector of crates.
    let categories_crates = fetch_all_categories(&client, &list_of_categories)
        .expect("Should fetch all crates of every category and return them");

    
    //Fetch each and every crates for each and every category to get the full details.   
    let category_crates_with_full_details = fetch_full_details(&client, &categories_crates)
        .expect("Should collect all the information of every crate");


    // Serialize the full crates, to .json files for each categorie. 
    for (categorie, krate   ) in category_crates_with_full_details.iter() { //iterate the collection 
        let vec_full_crates_to_json = serde_json::to_string_pretty(krate) //new vector that holds json string in pretty format.
        .expect("Error serializing");
        
        fs::write(format!("{}.json", categorie.to_string()), vec_full_crates_to_json)   //Macro to make the path of the file, and write the json vec on it
        .expect("Failed to write the file");

    }
    
}