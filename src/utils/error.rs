use super::MeshLevel;
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

    #[error("{0} is not lower than {1}")]
    InvalidMeshLevelForLowerLevel(MeshLevel, MeshLevel),

    #[error("Unsupported mesh level conversion from {0} to {1}")]
    UnsupportedMeshLevelConversion(MeshLevel, MeshLevel),

    #[error(
        "Mismathed levels: the level must be the same for meshcode_sw and meshcode_ne {0} != {1}"
    )]
    MismatchedMeshLevels(MeshLevel, MeshLevel),

    #[error("Parse Error: {0}")]
    ParseError(#[from] strum::ParseError),
}

pub type Result<T> = std::result::Result<T, JismeshError>;
