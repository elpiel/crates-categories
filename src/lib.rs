use std::{collections::HashMap};
use crates_io_api::{Crate, Error, SyncClient, FullCrate};


pub fn fetch_all_categories(client: &SyncClient, categories: &[&str]) -> Result<HashMap<String, Vec<Crate>>, Error> {
    let mut category_crates = HashMap::new();

    for category in categories {

        
       
        let all_crates_of_category = client.all_crates(Some(format!("category={}", category)))
        .expect("Error listing the crates");

        let previous = category_crates.insert(category.to_string(), all_crates_of_category);

    

        assert!(previous.is_none())
    }
    
    Ok(category_crates)

}


pub fn fetch_full_details(client: &SyncClient, categories_crate: &HashMap<String,Vec<Crate>> ) -> Result <HashMap<String, Vec<FullCrate>>, Error> {
    let mut category_full_data: HashMap<String, Vec<FullCrate>> = HashMap::new();
    let mut vec = Vec::new() ;
    for (category, krate) in categories_crate.iter(){
        for k in krate
        {
            let full_crate =  client.full_crate(&k.name, false)
            .expect("error collectin the full crate");
            vec.push(full_crate);
            println!("In category : {} Pushed a crate, Vec lenght {}",&category, vec.len()); //Just log for testing will delete later

        }
        category_full_data.insert(category.to_string(), vec.clone());
        vec.clear(); 
    }
   
    Ok(category_full_data)

}


   