use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum JismeshError {
    #[error("Latitude {0} is out of bounds (0 <= lat < 66.66)")]
    LatitudeOutOfBounds(f64),
    #[error("Longitude {0} is out of bounds (100 <= lon < 180)")]
    LongitudeOutOfBounds(f64),

    #[error("Invalid meshcode: cannot determine level for {0}")]
    UnknownMeshLevelForCode(u64),
    #[error("Invalid meshcode at level {0}: {1}")]
    InvalidMeshcodeAtLevel(usize, u64),

    #[error("Invalid mesh level: {0}")]
    InvalidMeshLevel(usize),
}

pub type Result<T> = std::result::Result<T, JismeshError>;
