use dotenvy::dotenv;
use gloo_net::http::Request;
use serde_json;
use shared::types::spell::QuerySpell;
use std::env;

pub async fn post(input_spell: QuerySpell) {
    dotenv().ok();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    // let base_url: String = env::var("BASE_URL").expect("BASE_URL must be set");
    let url = format!("{}/read_spells", base_url);

    let body = match serde_json::to_string(&input_spell) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to serialize input_spell: {}", err);
            return;
        }
    };

    // Make the POST request
    match Request::post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => {
            if response.ok() {
                match response.text().await {
                    Ok(text) => println!("Response: {}", text),
                    Err(err) => eprintln!("Failed to read response body: {}", err),
                }
            } else {
                eprintln!(
                    "Failed to post spell. HTTP Status: {}. Body: {:?}",
                    response.status(),
                    response
                        .text()
                        .await
                        .unwrap_or_else(|_| "No response body".to_string())
                );
            }
        }
        Err(err) => {
            eprintln!("Error making POST request: {}", err);
        }
    }
}
