use std::error::Error;

use config::{get_ciks_to_parse, get_sec_user_client};

mod config;

pub async fn retrieve_document() -> Result<(), Box<dyn Error>> {
    println!("User Client: {:?}", get_sec_user_client());
    println!("CIKs: {:?}", get_ciks_to_parse());
    println!("I retrieved something!");

    // println!("Will look for this company: {cik}");

    Ok(())
}
