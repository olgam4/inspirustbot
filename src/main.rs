use surf::{Config, Client, Url};

#[tokio::main]
async fn main() {
    let client: Client = Config::new()
        .set_base_url(Url::parse("https://inspirobot.me/api").unwrap())
        .try_into().unwrap();

    let session_id = get_session_id(client.clone()).await;

    let quote = get_flow_text(client.clone(), session_id).await;

    println!("{}", quote);
}

async fn get_session_id(client: Client) -> String {
    let mut response = client.get("?getSessionID=true").await.unwrap();
    let body = response.body_string().await.unwrap();
    body
}

async fn get_flow_text(client: Client, session_id: String) -> String {
    let mut response = client.get(format!("?generateFlow=1&sessionID={session_id}")).await.unwrap();
    let body = response.body_string().await.unwrap();
    let deserialized = serde_json::from_str::<serde_json::Value>(&body).unwrap();
    let quote = deserialized.get("data").unwrap()[1].get("text").unwrap();

    quote.as_str().unwrap().to_string()
}
