use std::collections::HashMap;

use crates_io_api::{Crate, Error, SyncClient, FullCrate};


pub fn fetch_all_categories(client: &SyncClient, categories: &[&str]) -> Result<HashMap<String, Vec<Crate>>, Error> {
    let mut category_crates = HashMap::new();

    for category in categories {

        // TODO: add crate urlencoded in Cargo.toml
        // let url_encoded_query = urlencoded
        // rust_analyzer - VS code
        let all_crates_of_category = client.all_crates(Some(format!("category={}", category)))
        .expect("Error listing the crates");

        let previous = category_crates.insert(category.to_string(), all_crates_of_category);

    

        assert!(previous.is_none(),)
    }
    
    Ok(category_crates)

}

pub fn fetch_full_details(client: &SyncClient, categories_crate: HashMap<&str,&Vec<Crate>> ) -> Result <HashMap<String, FullCrate>, Error> {
    let mut category_full_data: HashMap<String, FullCrate> = HashMap::new();
    
    for (category, krate) in categories_crate{
        for k in krate{
            let full_crate = client.full_crate(&k.name, false)
                .expect("Error fetching a full crate");
            category_full_data.insert(category.to_string(), full_crate);
            }
    }
   
    Ok(category_full_data)

}


   