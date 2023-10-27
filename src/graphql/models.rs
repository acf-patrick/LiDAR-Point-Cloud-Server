use juniper::*;

#[derive(GraphQLObject)]
pub struct LasHeaderVersion {
    pub minor: i32,
    pub major: i32,
}

impl From<las::Header> for LasHeaderVersion {
    fn from(value: las::Header) -> Self {
        let version = value.version();
        LasHeaderVersion {
            minor: version.minor.into(),
            major: version.major.into(),
        }
    }
}

#[derive(GraphQLObject)]
pub struct LasPointFormat {
    #[graphql(description = "true if includes GPS time")]
    pub gps_time: bool,

    #[graphql(description = "true if includes RGB triplets")]
    pub color: bool,

    #[graphql(description = "true if is compressed")]
    pub compressed: bool,
}

impl From<las::Header> for LasPointFormat {
    fn from(value: las::Header) -> Self {
        let format = value.point_format();
        LasPointFormat {
            color: format.has_gps_time,
            compressed: format.is_compressed,
            gps_time: format.has_gps_time,
        }
    }
}

#[derive(GraphQLObject)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<las::Vector<f64>> for Vector {
    fn from(value: las::Vector<f64>) -> Self {
        Vector {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(GraphQLObject)]
pub struct LasInfo {
    #[graphql(description = "This is often the flightline number")]
    pub file_source_id: i32,

    pub version: LasHeaderVersion,

    #[graphql(description = "Header's file creation date")]
    pub date: Option<String>,

    #[graphql(description = "Describes the attributes and extra bytes of each point")]
    pub point_format: LasPointFormat,

    #[graphql(description = "Scale that transforms XYZ coordinates")]
    pub scale: Vector,

    #[graphql(description = "Offset that transforms XYZ coordinates")]
    pub offset: Vector,

    #[graphql(description = "Minimum bound in three dimensions")]
    pub min: Vector,

    #[graphql(description = "Maximum bound in three dimensions")]
    pub max: Vector,

    pub number_of_points: String,
}

impl From<crate::database::models::File> for LasInfo {
    fn from(value: crate::database::models::File) -> Self {
        LasInfo {
            date: value.date,
            file_source_id: value.file_source_id,
            max: Vector {
                x: value.max_x.into(),
                y: value.max_y.into(),
                z: value.max_z.into(),
            },
            min: Vector {
                x: value.min_x.into(),
                y: value.min_y.into(),
                z: value.min_z.into(),
            },
            number_of_points: format!("{}", value.number_of_points),
            offset: Vector {
                x: value.offset_x.into(),
                y: value.offset_y.into(),
                z: value.offset_z.into(),
            },
            point_format: LasPointFormat {
                gps_time: value.has_gps_time > 0,
                color: value.has_color > 0,
                compressed: value.is_compressed > 0,
            },
            scale: Vector {
                x: value.scale_x.into(),
                y: value.scale_y.into(),
                z: value.scale_z.into(),
            },
            version: LasHeaderVersion {
                minor: value.version_minor,
                major: value.version_major,
            },
        }
    }
}

impl From<las::Header> for LasInfo {
    fn from(value: las::Header) -> Self {
        let transform = value.transforms();

        LasInfo {
            date: value.date().map(|date| date.to_string()),
            file_source_id: value.file_source_id().into(),
            max: Vector::from(value.bounds().max),
            min: Vector::from(value.bounds().min),
            number_of_points: format!("{}", value.number_of_points()),
            offset: Vector {
                x: transform.x.offset,
                y: transform.y.offset,
                z: transform.z.offset,
            },
            scale: Vector {
                x: transform.x.scale,
                y: transform.y.scale,
                z: transform.z.scale,
            },
            point_format: LasPointFormat::from(value.clone()),
            version: LasHeaderVersion::from(value),
        }
    }
}

#[derive(GraphQLObject)]
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
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

#[derive(GraphQLObject)]
pub struct Part {
    pub id: String,
    pub file_id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub edge: f64,
}

impl From<crate::database::models::Part> for Part {
    fn from(value: crate::database::models::Part) -> Self {
        Part {
            id: value.id,
            file_id: value.file_id,
            edge: value.edge.into(),
            x: value.x.into(),
            y: value.y.into(),
            z: value.z.into(),
        }
    }
}
