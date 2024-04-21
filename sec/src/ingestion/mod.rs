pub mod retrieval;
pub mod storage;

use std::error::Error;

use retrieval::run_retrieval_process;
use storage::store_to_disk;


pub async fn ingest_document() -> Result<(), Box<dyn Error>> {
    run_retrieval_process().await?;
    store_to_disk();
    Ok(())
}
