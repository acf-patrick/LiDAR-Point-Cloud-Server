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
