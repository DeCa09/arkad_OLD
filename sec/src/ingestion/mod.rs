pub mod retrieval;
pub mod storage;

use retrieval::retrieve_document;
use storage::store_to_disk;

pub fn ingest_document() {
    retrieve_document();
    store_to_disk();
}
