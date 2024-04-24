use sec::ingestion::ingest_document;
use sec::parser::parse_documents;

use std::error::Error;

#[tokio::main] // Attribute to run the async main function
async fn main() -> Result<(), Box<dyn Error>> {
    parse_documents();
    ingest_document().await?;
    Ok(())
}
