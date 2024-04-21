use reqwest::{self, Client, Error};

const USER_AGENT: &str = "Demir Catovic d.catovic9@gmail.com";
const CIK_LIST: [&str; 4] = ["0000320193", "0001067983", "0001045810", "0001326801"]; // AAPL, BRK, NVDA, META

pub fn get_sec_user_client() -> Result<Client, Error> {
    let client = reqwest::Client::builder()
        .user_agent(get_sec_user_agent())
        .build()?;
    Ok(client)
}

fn get_sec_user_agent() -> &'static str {
    USER_AGENT
}

pub fn get_ciks_to_parse() -> &'static [&'static str] {
    &CIK_LIST
}
