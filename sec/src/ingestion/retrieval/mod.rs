use reqwest::Error;

use super::RawCompanyData;
use config::{get_ciks_to_parse, get_sec_user_client};

mod config;

pub async fn retrieve_document(cik: &str) -> Result<RawCompanyData, Error> {
    let url = format!("https://data.sec.gov/api/xbrl/companyfacts/CIK{cik}.json");

    let client = get_sec_user_client()?;

    let body = client.get(&url).send().await?.text().await?;

    println!(
        "Did the retrieval process for this cik: {cik} with this body: {}...",
        &body[..100]
    );

    let company_data = RawCompanyData {
        id: cik.to_string(),
        data: body,
    };

    Ok(company_data)
}

pub async fn run_retrieval_process() -> Result<Vec<RawCompanyData>, Error> {
    let ciks = get_ciks_to_parse();

    let mut raw_data_collection = Vec::new();

    for cik in ciks {
        raw_data_collection.push(retrieve_document(cik).await?);
    }

    Ok(raw_data_collection)
}
