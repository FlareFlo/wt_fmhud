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
    let indicators = reqwest::get("http://localhost:8111/indicators").await.unwrap();
	let state = reqwest::get("http://localhost:8111/state").await.unwrap();
    let json: Indicator = serde_json::from_str(&indicators.text().await.unwrap()).unwrap();
	println!("{:#?}", json);
}
