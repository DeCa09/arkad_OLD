pub mod retrieval;
pub mod storage;

use retrieval::get_user_agent;
use storage::store_to_disk;

pub fn retrieve_documents(){
    println!("I retrieved something!");
    println!("Here is the User Agent we use for retrieval: {}", get_user_agent());
}

pub fn store_documents(){
    store_to_disk();
}
