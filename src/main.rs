//use crates_categories::fetch_all_categories;
use crates_io_api::{SyncClient, Error};


fn list_top_dependencies() -> Result<(), Error> {
    // Instantiate the client.
    let client = SyncClient::new(
         "my-user-agent (my-contact@domain.com)",
         std::time::Duration::from_millis(1000),
    )?;
    // Retrieve summary data.
    let summary = client.summary()?;
    for c in summary.most_downloaded {
        println!("{}:", c.id);
    }
    Ok(())        
}


fn main() {
    
    //let list_of_categories: [&str; 0] = [];
    //let categories = fetch_all_categories(&list_of_categories);

    //for c in categories {
    //    println!("{}:", c)
    //}


    println!("Call lib.rs and save results to a file or multiple files)!");

    match list_top_dependencies() {
        Ok(()) => println!("I worked and printed the crates :D "),
        Err(err) => eprintln!("error: not again :( {}", err),
    };

}


