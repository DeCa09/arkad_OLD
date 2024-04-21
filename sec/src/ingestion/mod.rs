pub mod retrieval;
pub mod storage;

use std::error::Error;

use retrieval::retrieve_document;
use storage::store_to_disk;

pub async fn ingest_document() -> Result<(), Box<dyn Error>> {
    retrieve_document().await?;
    store_to_disk();
    Ok(())
}
