use crate::response::Indicator;

mod response;

#[tokio::main]
async fn main() {
    let reply = reqwest::get("http://localhost:8111/indicators").await.unwrap();
    let json: Indicator = serde_json::from_str(&reply.text().await.unwrap()).unwrap();
	println!("{:#?}", json);
}
