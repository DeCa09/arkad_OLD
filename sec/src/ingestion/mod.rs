pub mod retrieval;
pub mod storage;

use std::error::Error;

use retrieval::run_retrieval_process;
use storage::store_to_disk;

pub trait Serializable: AsRef<[u8]> {
    fn get_id(&self) -> &String;
    fn as_ref(&self) -> &[u8];
}
pub struct RawCompanyData {
    id: String,
    data: String,
}

impl AsRef<[u8]> for RawCompanyData {
    fn as_ref(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl Serializable for RawCompanyData {
    fn as_ref(&self) -> &[u8] {
        self.data.as_bytes()
    }
    fn get_id(&self) -> &String {
        &self.id
    }
}

pub async fn ingest_document() -> Result<(), Box<dyn Error>> {
    let raw_company_data_collection = run_retrieval_process().await?;
    store_to_disk(raw_company_data_collection)?;

    Ok(())
}
