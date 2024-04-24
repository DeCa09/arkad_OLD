use std::error::Error;
use std::fs::write;

mod config;

use super::Serializable;
use config::get_output_directory;

pub fn store_to_disk<T: Serializable>(
    company_data_collection: Vec<T>,
) -> Result<(), Box<dyn Error>> {
    let output_dir = get_output_directory();

    for company_data in company_data_collection {
        let id = company_data.get_id();
        write(format!("{output_dir}/{id}.json"), company_data)?;
    }
    println!("I stored something!");
    Ok(())
}
