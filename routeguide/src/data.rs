use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

#[allow(dead_code)]
pub fn load() -> Vec<crate::routeguide::Feature> {
    let file = File::open("data/route_guide_db.json").expect("failed to open data file");

    let decode: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");

    decode
        .into_iter()
        .map(|feature| crate::routeguide::Feature {
            name: feature.name,
            location: Some(crate::routeguide::Point {
                latitude: feature.location.latitude,
                longitude: feature.location.longitude,
            }),
        })
        .collect()
}
