use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    data: Data,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    wave: Vec<Wave>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wave {
    timestamp: u64,
    utc_offset: i128,
    surf: Surf,
    swells: Vec<Swell>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Surf {
    min: f64,
    max: f64,
    optimal_score: i8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Swell {
    height: f64,
    period: f64,
    direction: f64,
    direction_min: f64,
    optimal_score: i8,
}

// #[derive(Deserialize, Debug)]
// struct Spot {
//     pub name: Option<String>,
//     pub wave_height: Option<String>,
//     pub tide: Option<String>,
//     pub wind: Option<String>,
//     pub timestamp: Option<DateTime<Utc>>,
// }
