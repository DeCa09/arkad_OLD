use sec::parser::parse_documents;
use sec::ingestion::{retrieve_documents, store_documents};

fn main(){
    parse_documents();
    retrieve_documents();
    store_documents();
}
