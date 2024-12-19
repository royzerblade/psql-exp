use gloo_net::http::Request;
use shared::types::spell::QuerySpell;
use std::env;

pub async fn get() -> Vec<QuerySpell> {
    let base_url: String = env::var("BASE_URL").expect("DATABASE_URL must be set");
    let url = format!("{}/read_spells", base_url);
    let response: Vec<QuerySpell> = Request::get(&url)
        .send()
        .await
        .expect("Failed to send request")
        .json::<Vec<QuerySpell>>()
        .await
        .expect("Failed to deserialize response");

    return response;
}
