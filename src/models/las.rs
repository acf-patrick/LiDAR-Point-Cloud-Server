use juniper::*;

#[derive(GraphQLObject)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    intensity: i32,
    point_source_id: i32,
    gps_time: Option<f64>,
    classification: Classification,
    color: Option<Color>,
}

impl From<las::Point> for Point {
    fn from(value: las::Point) -> Self {
        Point {
            x: value.x,
            y: value.y,
            z: value.z,
            intensity: value.intensity.into(),
            classification: value.classification.into(),
            point_source_id: value.point_source_id.into(),
            color: if value.color.is_none() {
                Option::<Color>::None
            } else {
                Some(value.color.unwrap().into())
            },
            gps_time: value.gps_time,
        }
    }
}

impl Point {
    pub fn new() -> Point {
        Point {
            classification: Classification::Unclassified,
            color: Some(Color {
                red: 0xff,
                green: 0,
                blue: 0,
            }),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            intensity: 0,
            gps_time: None,
            point_source_id: 0,
        }
    }
}

#[derive(GraphQLObject)]
struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl From<las::Color> for Color {
    fn from(value: las::Color) -> Self {
        Color {
            red: value.red.into(),
            green: value.green.into(),
            blue: value.blue.into(),
        }
    }
}

/// The ASPRS classification table
#[derive(GraphQLEnum)]
pub enum Classification {
    CreatedNeverClassified,
    Unclassified,
    Ground,
    LowVegetation,
    MediumVegetation,
    HighVegetation,
    Building,
    LowPoint,
    ModelKeyPoint,
    Water,
    Rail,
    RoadSurface,
    WireGuard,
    WireConductor,
    TransmissionTower,
    WireStructureConnector,
    BridgeDeck,
    HighNoise,
}

impl From<las::point::Classification> for Classification {
    fn from(value: las::point::Classification) -> Self {
        match value {
            las::point::Classification::HighNoise => Classification::HighNoise,
            las::point::Classification::BridgeDeck => Classification::BridgeDeck,
            las::point::Classification::WireStructureConnector => {
                Classification::WireStructureConnector
            }
            las::point::Classification::TransmissionTower => Classification::TransmissionTower,
            las::point::Classification::WireConductor => Classification::WireConductor,
            las::point::Classification::WireGuard => Classification::WireGuard,
            las::point::Classification::RoadSurface => Classification::RoadSurface,
            las::point::Classification::Rail => Classification::Rail,
            las::point::Classification::Water => Classification::Water,
            las::point::Classification::ModelKeyPoint => Classification::ModelKeyPoint,
            las::point::Classification::LowPoint => Classification::LowPoint,
            las::point::Classification::Building => Classification::Building,
            las::point::Classification::HighVegetation => Classification::HighVegetation,
            las::point::Classification::MediumVegetation => Classification::MediumVegetation,
            las::point::Classification::LowVegetation => Classification::LowVegetation,
            las::point::Classification::Ground => Classification::Ground,
            las::point::Classification::CreatedNeverClassified => {
                Classification::CreatedNeverClassified
            }
            _ => Classification::Unclassified,
        }
    }
}
