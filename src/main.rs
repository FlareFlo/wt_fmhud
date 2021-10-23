use crate::response::Indicator;

mod response;

/*
Routes:
/mission.json
/map_obj.json
/map_info.json
/indicators
/state
 */

#[tokio::main]
async fn main() {
    let reply = reqwest::get("http://localhost:8111/indicators").await.unwrap();
    let json: Indicator = serde_json::from_str(&reply.text().await.unwrap()).unwrap();
	println!("{:#?}", json);
}
