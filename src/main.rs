//use crates_categories::fetch_all_categories;
use crates_io_api::{SyncClient, Error, ListOptions, Sort};
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

fn main()  {
    
    //Have a list of categories (configuration) to fetch from crates.io
    let list_of_categories: [&str; 2] = ["algorithms", "asynchronous"];
    
    //create a SyncClient
    

    let client = SyncClient::new(
        "IPCA - Rust",
        std::time::Duration::from_millis(1000), //Resquests to do per second. 
    ).expect("Error making the client");
    
    let category_crates = HashMap::new();

    for category in list_of_categories {
        let crates = client.crates(ListOptions{
            sort: Sort::Alphabetical,
            per_page: 100,
            page: 1,
            query: Some(format!("category={}", category)),
          }).expect("Error listing the crates");
        
    }
    


    match list_top_dependencies() {
        Ok(()) => println!("I worked and printed the crates :D "),
        Err(err) => eprintln!("error: not again :( {}", err),
    };
    
    //let categories = fetch_all_categories(&list_of_categories);
    //println!("Call lib.rs and save results to a file or multiple files)!");
}


