use reqwest::Error;

use config::{get_ciks_to_parse, get_sec_user_client};

mod config;


pub async fn retrieve_document(cik: &str) -> Result<String, Error> {
    let url = format!("https://data.sec.gov/api/xbrl/companyfacts/CIK{cik}.json");
    
    let client = get_sec_user_client()?;

    let body = client.get(&url).send().await?.text().await?;

    println!("Did the retrieval process for this cik: {cik} with this body: {}...", &body[..100]);

    Ok(body)
}

pub async fn run_retrieval_process() -> Result<(), Error>{
    let ciks = get_ciks_to_parse();

    for cik in ciks {
        retrieve_document(cik).await?;
    }

    Ok(())

}
