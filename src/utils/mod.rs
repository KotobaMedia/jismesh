mod error;
mod levels;
pub use error::JismeshError;
use error::Result;
pub use levels::MeshLevel;
mod meshcode;
pub use meshcode::to_meshcode;
mod meshlevel;
pub use meshlevel::to_meshlevel;
mod meshpoint;
pub use meshpoint::to_meshpoint;
mod envelope;
pub use envelope::{to_envelope, to_intersects};
use ndarray::Array1;

const UNIT_LAT_LV1: f64 = 2.0 / 3.0;
const UNIT_LON_LV1: f64 = 1.0;
const UNIT_LAT_40000: f64 = UNIT_LAT_LV1 / 2.0;
const UNIT_LON_40000: f64 = UNIT_LON_LV1 / 2.0;
const UNIT_LAT_20000: f64 = UNIT_LAT_40000 / 2.0;
const UNIT_LON_20000: f64 = UNIT_LON_40000 / 2.0;
const UNIT_LAT_16000: f64 = UNIT_LAT_LV1 / 5.0;
const UNIT_LON_16000: f64 = UNIT_LON_LV1 / 5.0;
const UNIT_LAT_LV2: f64 = UNIT_LAT_LV1 / 8.0;
const UNIT_LON_LV2: f64 = UNIT_LON_LV1 / 8.0;
const UNIT_LAT_8000: f64 = UNIT_LAT_LV1 / 10.0;
const UNIT_LON_8000: f64 = UNIT_LON_LV1 / 10.0;
const UNIT_LAT_5000: f64 = UNIT_LAT_LV2 / 2.0;
const UNIT_LON_5000: f64 = UNIT_LON_LV2 / 2.0;
const UNIT_LAT_4000: f64 = UNIT_LAT_8000 / 2.0;
const UNIT_LON_4000: f64 = UNIT_LON_8000 / 2.0;
const UNIT_LAT_2500: f64 = UNIT_LAT_5000 / 2.0;
const UNIT_LON_2500: f64 = UNIT_LON_5000 / 2.0;
const UNIT_LAT_2000: f64 = UNIT_LAT_LV2 / 5.0;
const UNIT_LON_2000: f64 = UNIT_LON_LV2 / 5.0;
const UNIT_LAT_LV3: f64 = UNIT_LAT_LV2 / 10.0;
const UNIT_LON_LV3: f64 = UNIT_LON_LV2 / 10.0;
const UNIT_LAT_LV4: f64 = UNIT_LAT_LV3 / 2.0;
const UNIT_LON_LV4: f64 = UNIT_LON_LV3 / 2.0;
const UNIT_LAT_LV5: f64 = UNIT_LAT_LV4 / 2.0;
const UNIT_LON_LV5: f64 = UNIT_LON_LV4 / 2.0;
const UNIT_LAT_LV6: f64 = UNIT_LAT_LV5 / 2.0;
const UNIT_LON_LV6: f64 = UNIT_LON_LV5 / 2.0;

pub(crate) fn unit_lat_lon(level: MeshLevel) -> (f64, f64) {
    match level {
        MeshLevel::Lv1 => (UNIT_LAT_LV1, UNIT_LON_LV1),
        MeshLevel::X40 => (UNIT_LAT_40000, UNIT_LON_40000),
        MeshLevel::X20 => (UNIT_LAT_20000, UNIT_LON_20000),
        MeshLevel::X16 => (UNIT_LAT_16000, UNIT_LON_16000),
        MeshLevel::Lv2 => (UNIT_LAT_LV2, UNIT_LON_LV2),
        MeshLevel::X8 => (UNIT_LAT_8000, UNIT_LON_8000),
        MeshLevel::X5 => (UNIT_LAT_5000, UNIT_LON_5000),
        MeshLevel::X4 => (UNIT_LAT_4000, UNIT_LON_4000),
        MeshLevel::X2_5 => (UNIT_LAT_2500, UNIT_LON_2500),
        MeshLevel::X2 => (UNIT_LAT_2000, UNIT_LON_2000),
        MeshLevel::Lv3 => (UNIT_LAT_LV3, UNIT_LON_LV3),
        MeshLevel::Lv4 => (UNIT_LAT_LV4, UNIT_LON_LV4),
        MeshLevel::Lv5 => (UNIT_LAT_LV5, UNIT_LON_LV5),
        MeshLevel::Lv6 => (UNIT_LAT_LV6, UNIT_LON_LV6),
    }
}

pub(crate) fn unit_lat(level: MeshLevel) -> f64 {
    unit_lat_lon(level).0
}

pub(crate) fn unit_lon(level: MeshLevel) -> f64 {
    unit_lat_lon(level).1
}

pub(crate) fn slice(codes: &Array1<u64>, start: u32, stop: u32) -> Array1<u8> {
    codes.mapv(|t| {
        let num_digits = (t as f64).log10().floor() as u32 + 1;
        if num_digits < stop {
            0
        } else {
            let mask1 = 10_u64.pow(num_digits - start);
            let mask2 = 10_u64.pow(num_digits - stop);
            ((t % mask1) / mask2) as u8
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_slice() {
        // Test single digit extraction
        assert_eq!(slice(&array![12345], 0, 1), array![1]); // Leftmost digit
        assert_eq!(slice(&array![12345], 1, 2), array![2]); // Second digit from left
        assert_eq!(slice(&array![12345], 4, 5), array![5]); // Rightmost digit

        // Test multiple digit extraction
        assert_eq!(slice(&array![12345], 0, 2), array![12]); // Two leftmost digits
        assert_eq!(slice(&array![12345], 1, 4), array![234]); // Middle digits
        assert_eq!(slice(&array![12345], 3, 5), array![45]); // Two rightmost digits

        // Test with multiple codes
        assert_eq!(slice(&array![123, 456, 7890], 0, 1), array![1, 4, 7]);
        assert_eq!(slice(&array![123, 456, 7890], 1, 3), array![23, 56, 89]);

        // Test with zero
        assert_eq!(slice(&array![0], 0, 1), array![0]);

        // Edge cases
        assert_eq!(slice(&array![5], 0, 1), array![5]); // Single digit
        assert_eq!(slice(&array![5], 2, 2), array![0]); // Out of bounds
        assert_eq!(slice(&array![12345], 6, 7), array![0]); // Beyond digits available
    }
}
