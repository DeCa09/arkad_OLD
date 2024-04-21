pub mod retrieval;


use retrieval::get_user_agent;

pub fn retrieve_documents(){
    println!("I retrieved something!");
    println!("Here is the User Agent we use for retrieval: {}", get_user_agent());
}
