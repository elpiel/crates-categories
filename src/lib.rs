use std::collections::HashMap;

use crates_io_api::{Crate, Error, SyncClient};


pub fn fetch_all_categories(client: &SyncClient, categories: &[&str]) -> Result<HashMap<String, Vec<Crate>>, Error> {
    let mut category_crates = HashMap::new();

    for category in categories {

        // TODO: add crate urlencoded in Cargo.toml
        // let url_encoded_query = urlencoded
        // rust_analyzer - VS code
        let all_crates_of_category = client.all_crates(Some(format!("category={}", category)))
        .expect("Error listing the crates");

        let previous = category_crates.insert(category.to_string(), all_crates_of_category);

        assert!(previous.is_none())
    }
    
    Ok(category_crates)

}