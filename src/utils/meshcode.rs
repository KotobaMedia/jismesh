use super::*;
use crate::utils::error::JismeshError;
use std::{fmt, ops::Deref, str::FromStr};

/// 地域メッシュコードを表す構造体
///
/// TryFrom<u64> を実装しているので u64 から MeshCode への変換に使ってください。
/// Into<u64> も実装しているので、 u64 として利用する場合は使ってください。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct MeshCode {
    pub(crate) value: u64,
    pub level: MeshLevel,
}

impl MeshCode {
    /// あるメッシュコードの次数を下げる（親メッシュコードを取得する）ために使ってください。
    /// 現在は、 Lv3 -> Lv2 -> Lv1 のみ対応しております。
    pub fn lower_level(&self, level: MeshLevel) -> Result<MeshCode> {
        if level > self.level {
            return Err(JismeshError::InvalidMeshLevelForLowerLevel(
                self.level, level,
            ));
        }

        let new_value = match (self.level, level) {
            (x, y) if x == y => Ok(self.value),
            (MeshLevel::Lv3, MeshLevel::Lv2) => Ok(self.value / 100),
            (MeshLevel::Lv2, MeshLevel::Lv1) => Ok(self.value / 100),
            (MeshLevel::Lv3, MeshLevel::Lv1) => Ok(self.value / 10000),
            _ => Err(JismeshError::UnsupportedMeshLevelConversion(
                self.level, level,
            )),
        }?;

        Ok(MeshCode {
            value: new_value,
            level,
        })
    }

    /// メッシュコードから緯度経度の座標を取得する。
    /// 緯度経度の座標は、lat/lon_multiplier で位置を調整できます。
    /// lat: 0.0, lon: 0.0 の場合は、メッシュコードの SW (南西) 端の座標を返します。
    /// lat: 1.0, lon: 1.0 の場合は、メッシュコードの NE (北東) 端の座標を返します。
    /// lat: 0.5, lon: 0.5 の場合は、メッシュコードの中央の座標を返します。
    /// 返却値は (緯度, 経度) です。
    pub fn point(&self, lat_multiplier: f64, lon_multiplier: f64) -> Result<(f64, f64)> {
        let points = to_meshpoint(&[self.value], &[lat_multiplier], &[lon_multiplier])?;
        Ok((points[0][0], points[1][0]))
    }

    /// メッシュコードが指定されたメッシュコードを含むかどうかを確認する。
    pub fn contains(&self, code: &MeshCode) -> bool {
        if self.level == code.level {
            return self.value == code.value;
        }
        if self.level > code.level {
            return false;
        }

        // Check if the code is a lower level of this mesh code
        let parent_code = code.lower_level(self.level);
        match parent_code {
            Ok(parent) => self.value == parent.value,
            Err(_) => false,
        }
    }

    /// メッシュコードが指定されたメッシュコードと交差するかどうかを確認する。
    pub fn intersects(&self, other: &MeshCode) -> bool {
        if self.level < other.level {
            self.contains(other)
        } else {
            other.contains(self)
        }
    }
}

impl TryFrom<u64> for MeshCode {
    type Error = error::JismeshError;

    fn try_from(value: u64) -> Result<Self> {
        let level = to_meshlevel(&[value])?
            .first()
            .cloned()
            .ok_or(JismeshError::UnknownMeshLevelForCode(value))?;
        Ok(MeshCode { value, level })
    }
}

impl FromStr for MeshCode {
    type Err = error::JismeshError;

    fn from_str(value: &str) -> Result<Self> {
        let value = value
            .parse::<u64>()
            .map_err(|_| JismeshError::InvalidMeshCode(value.to_string()))?;
        value.try_into()
    }
}

impl From<MeshCode> for u64 {
    fn from(meshcode: MeshCode) -> Self {
        meshcode.value
    }
}

impl fmt::Display for MeshCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq<u64> for MeshCode {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}

/// Converts latitude & longitude to a meshcode.
/// 緯度経度から指定次の地域メッシュコードを算出する。
///
/// Args:
/// * lat: 世界測地系の緯度(度単位)
/// * lon: 世界測地系の経度(度単位)
pub fn to_meshcode(lat: &[f64], lon: &[f64], level: MeshLevel) -> Result<Vec<MeshCode>> {
    // Validate bounds for all values in the arrays
    for &lat_val in lat.iter() {
        if !(0.0..66.66).contains(&lat_val) {
            return Err(JismeshError::LatitudeOutOfBounds(lat_val));
        }
    }

    for &lon_val in lon.iter() {
        if !(100.0..180.0).contains(&lon_val) {
            return Err(JismeshError::LongitudeOutOfBounds(lon_val));
        }
    }

    // Create output vector
    let result_len = lat.len().max(lon.len());
    let mut result = Vec::with_capacity(result_len);

    for i in 0..result_len {
        let lat_val = lat[i % lat.len()];
        let lon_val = lon[i % lon.len()];

        // Calculate mesh code based on level
        let meshcode = match level {
            MeshLevel::Lv1 => meshcode_lv1(lat_val, lon_val),
            MeshLevel::X40 => meshcode_40000(lat_val, lon_val),
            MeshLevel::X20 => meshcode_20000(lat_val, lon_val),
            MeshLevel::X16 => meshcode_16000(lat_val, lon_val),
            MeshLevel::Lv2 => meshcode_lv2(lat_val, lon_val),
            MeshLevel::X8 => meshcode_8000(lat_val, lon_val),
            MeshLevel::X5 => meshcode_5000(lat_val, lon_val),
            MeshLevel::X4 => meshcode_4000(lat_val, lon_val),
            MeshLevel::X2_5 => meshcode_2500(lat_val, lon_val),
            MeshLevel::X2 => meshcode_2000(lat_val, lon_val),
            MeshLevel::Lv3 => meshcode_lv3(lat_val, lon_val),
            MeshLevel::Lv4 => meshcode_lv4(lat_val, lon_val),
            MeshLevel::Lv5 => meshcode_lv5(lat_val, lon_val),
            MeshLevel::Lv6 => meshcode_lv6(lat_val, lon_val),
        };
        result.push(meshcode);
    }

    Ok(result)
}

// Helper functions for calculating meshcodes at various levels
fn meshcode_lv1(lat: f64, lon: f64) -> MeshCode {
    let rem_lat_lv0 = lat;
    let rem_lon_lv0 = lon % 100.0;
    let ab = (rem_lat_lv0 / UNIT_LAT_LV1) as u64;
    let cd = (rem_lon_lv0 / UNIT_LON_LV1) as u64;
    MeshCode {
        value: ab * 100 + cd,
        level: MeshLevel::Lv1,
    }
}

fn meshcode_40000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv1(lat, lon).value;
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_40000) as u64 * 2 + (rem_lon_lv1 / UNIT_LON_40000) as u64 + 1;
    MeshCode {
        value: base * 10 + e,
        level: MeshLevel::X40,
    }
}

fn meshcode_20000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_40000(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let rem_lat_40000 = rem_lat_lv1 % UNIT_LAT_40000;
    let rem_lon_40000 = rem_lon_lv1 % UNIT_LON_40000;
    let f =
        (rem_lat_40000 / UNIT_LAT_20000) as u64 * 2 + (rem_lon_40000 / UNIT_LON_20000) as u64 + 1;
    let g = 5;
    MeshCode {
        value: base.value * 100 + f * 10 + g,
        level: MeshLevel::X20,
    }
}

fn meshcode_16000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_16000) as u64 * 2;
    let f = (rem_lon_lv1 / UNIT_LON_16000) as u64 * 2;
    let g = 7;
    MeshCode {
        value: base.value * 1000 + e * 100 + f * 10 + g,
        level: MeshLevel::X16,
    }
}

fn meshcode_lv2(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_LV2) as u64;
    let f = (rem_lon_lv1 / UNIT_LON_LV2) as u64;
    MeshCode {
        value: base.value * 100 + e * 10 + f,
        level: MeshLevel::Lv2,
    }
}

fn meshcode_8000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_8000) as u64;
    let f = (rem_lon_lv1 / UNIT_LON_8000) as u64;
    let g = 6;
    MeshCode {
        value: base.value * 1000 + e * 100 + f * 10 + g,
        level: MeshLevel::X8,
    }
}

fn meshcode_5000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv2(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let g = (rem_lat_lv2 / UNIT_LAT_5000) as u64 * 2 + (rem_lon_lv2 / UNIT_LON_5000) as u64 + 1;
    MeshCode {
        value: base.value * 10 + g,
        level: MeshLevel::X5,
    }
}

fn meshcode_4000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_8000(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let rem_lat_8000 = rem_lat_lv1 % UNIT_LAT_8000;
    let rem_lon_8000 = rem_lon_lv1 % UNIT_LON_8000;
    let h = (rem_lat_8000 / UNIT_LAT_4000) as u64 * 2 + (rem_lon_8000 / UNIT_LON_4000) as u64 + 1;
    let i = 7;
    MeshCode {
        value: base.value * 100 + h * 10 + i,
        level: MeshLevel::X4,
    }
}

fn meshcode_2500(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_5000(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let rem_lat_5000 = rem_lat_lv2 % UNIT_LAT_5000;
    let rem_lon_5000 = rem_lon_lv2 % UNIT_LON_5000;
    let h = (rem_lat_5000 / UNIT_LAT_2500) as u64 * 2 + (rem_lon_5000 / UNIT_LON_2500) as u64 + 1;
    let i = 6;
    MeshCode {
        value: base.value * 100 + h * 10 + i,
        level: MeshLevel::X2_5,
    }
}

fn meshcode_2000(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv2(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let g = (rem_lat_lv2 / UNIT_LAT_2000) as u64 * 2;
    let h = (rem_lon_lv2 / UNIT_LON_2000) as u64 * 2;
    let i = 5;
    MeshCode {
        value: base.value * 1000 + g * 100 + h * 10 + i,
        level: MeshLevel::X2,
    }
}

fn meshcode_lv3(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv2(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let g = (rem_lat_lv2 / UNIT_LAT_LV3) as u64;
    let h = (rem_lon_lv2 / UNIT_LON_LV3) as u64;
    MeshCode {
        value: base.value * 100 + g * 10 + h,
        level: MeshLevel::Lv3,
    }
}

fn meshcode_lv4(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv3(lat, lon);
    let rem_lat_lv3 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2 % UNIT_LAT_LV3;
    let rem_lon_lv3 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2 % UNIT_LON_LV3;
    let i = (rem_lat_lv3 / UNIT_LAT_LV4) as u64 * 2 + (rem_lon_lv3 / UNIT_LON_LV4) as u64 + 1;
    MeshCode {
        value: base.value * 10 + i,
        level: MeshLevel::Lv4,
    }
}

fn meshcode_lv5(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv4(lat, lon);
    let rem_lat_lv4 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2 % UNIT_LAT_LV3 % UNIT_LAT_LV4;
    let rem_lon_lv4 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2 % UNIT_LON_LV3 % UNIT_LON_LV4;
    let j = (rem_lat_lv4 / UNIT_LAT_LV5) as u64 * 2 + (rem_lon_lv4 / UNIT_LON_LV5) as u64 + 1;
    MeshCode {
        value: base.value * 10 + j,
        level: MeshLevel::Lv5,
    }
}

fn meshcode_lv6(lat: f64, lon: f64) -> MeshCode {
    let base = meshcode_lv5(lat, lon);
    let rem_lat_lv5 =
        lat % UNIT_LAT_LV1 % UNIT_LAT_LV2 % UNIT_LAT_LV3 % UNIT_LAT_LV4 % UNIT_LAT_LV5;
    let rem_lon_lv5 =
        lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2 % UNIT_LON_LV3 % UNIT_LON_LV4 % UNIT_LON_LV5;
    let k = (rem_lat_lv5 / UNIT_LAT_LV6) as u64 * 2 + (rem_lon_lv5 / UNIT_LON_LV6) as u64 + 1;
    MeshCode {
        value: base.value * 10 + k,
        level: MeshLevel::Lv6,
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_error_invalid_latitude_min() {
        let res = to_meshcode(&[-0.1], &[139.745433], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_error_invalid_latitude_max() {
        let res = to_meshcode(&[66.66], &[139.745433], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_error_invalid_longitude_min() {
        let res = to_meshcode(&[35.658581], &[99.99], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_error_invalid_longitude_max() {
        let res = to_meshcode(&[35.658581], &[180.0], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_tokyo_meshcodes() {
        let lat = [35.658581];
        let lon = [139.745433];
        let cases = vec![
            (MeshLevel::Lv1, 5339),
            (MeshLevel::X40, 53392),
            (MeshLevel::X20, 5339235),
            (MeshLevel::X16, 5339467),
            (MeshLevel::Lv2, 533935),
            (MeshLevel::X8, 5339476),
            (MeshLevel::X5, 5339354),
            (MeshLevel::X4, 533947637),
            (MeshLevel::X2_5, 533935446),
            (MeshLevel::X2, 533935885),
            (MeshLevel::Lv3, 53393599),
            (MeshLevel::Lv4, 533935992),
            (MeshLevel::Lv5, 5339359921),
            (MeshLevel::Lv6, 53393599212),
        ];
        for (level, expected) in cases {
            let result = to_meshcode(&lat, &lon, level).map(|code| code.first().unwrap().value);
            assert_eq!(result, Ok(expected), "Failed for level {:?}", level);
        }
    }

    #[test]
    fn test_kyoto_meshcodes() {
        let lat = [34.987574];
        let lon = [135.759363];
        let cases = vec![
            (MeshLevel::Lv1, 5235),
            (MeshLevel::X40, 52352),
            (MeshLevel::X20, 5235245),
            (MeshLevel::X16, 5235467),
            (MeshLevel::Lv2, 523536),
            (MeshLevel::X8, 5235476),
            (MeshLevel::X5, 5235363),
            (MeshLevel::X4, 523547647),
            (MeshLevel::X2_5, 523536336),
            (MeshLevel::X2, 523536805),
            (MeshLevel::Lv3, 52353680),
            (MeshLevel::Lv4, 523536804),
            (MeshLevel::Lv5, 5235368041),
            (MeshLevel::Lv6, 52353680412),
        ];
        for (level, expected) in cases {
            let result = to_meshcode(&lat, &lon, level).map(|code| code.first().unwrap().value);
            assert_eq!(result, Ok(expected), "Failed for level {:?}", level);
        }
    }

    #[test]
    fn test_meshcode_try_from_u64() {
        // Test Level 1 mesh code
        let meshcode = MeshCode::try_from(5339).unwrap();
        assert_eq!(meshcode.value, 5339);
        assert_eq!(meshcode.level, MeshLevel::Lv1);

        // Test Level 2 mesh code
        let meshcode = MeshCode::try_from(533935).unwrap();
        assert_eq!(meshcode.value, 533935);
        assert_eq!(meshcode.level, MeshLevel::Lv2);

        // Test Level 3 mesh code
        let meshcode = MeshCode::try_from(53393599).unwrap();
        assert_eq!(meshcode.value, 53393599);
        assert_eq!(meshcode.level, MeshLevel::Lv3);
    }

    #[test]
    fn test_meshcode_from_meshcode_to_u64() {
        let meshcode = MeshCode {
            value: 5339,
            level: MeshLevel::Lv1,
        };
        let value: u64 = meshcode.into();
        assert_eq!(value, 5339);

        let meshcode = MeshCode {
            value: 533935,
            level: MeshLevel::Lv2,
        };
        let value: u64 = meshcode.into();
        assert_eq!(value, 533935);
    }

    #[test]
    fn test_meshcode_to_lower_same_level() {
        let meshcode = MeshCode {
            value: 53393599,
            level: MeshLevel::Lv3,
        };
        let result = meshcode.lower_level(MeshLevel::Lv3).unwrap();
        assert_eq!(result.value, 53393599);
        assert_eq!(result.level, MeshLevel::Lv3);
    }

    #[test]
    fn test_meshcode_lower_levels() {
        let test_cases = vec![
            (MeshLevel::Lv3, MeshLevel::Lv2, 45304421, 453044),
            (MeshLevel::Lv2, MeshLevel::Lv1, 453044, 4530),
            (MeshLevel::Lv3, MeshLevel::Lv1, 45304421, 4530),
        ];
        for (from, to, input_value, expected_value) in test_cases {
            let meshcode = MeshCode::try_from(input_value).unwrap();
            assert_eq!(meshcode.level, from);

            let result = meshcode.lower_level(to).unwrap();
            assert_eq!(result.value, expected_value);
            assert_eq!(result.level, to);
        }
    }

    #[test]
    fn test_meshcode_to_lower_invalid_higher_level() {
        let meshcode = MeshCode {
            value: 5339,
            level: MeshLevel::Lv1,
        };
        let result = meshcode.lower_level(MeshLevel::Lv2);
        assert!(result.is_err());
        match result.unwrap_err() {
            JismeshError::InvalidMeshLevelForLowerLevel(from, to) => {
                assert_eq!(from, MeshLevel::Lv1);
                assert_eq!(to, MeshLevel::Lv2);
            }
            _ => panic!("Expected InvalidMeshLevelForLowerLevel error"),
        }
    }

    #[test]
    fn test_meshcode_to_lower_unsupported_conversion() {
        let meshcode = MeshCode {
            value: 53392,
            level: MeshLevel::X40,
        };
        let result = meshcode.lower_level(MeshLevel::Lv1);
        assert!(result.is_err());
        match result.unwrap_err() {
            JismeshError::UnsupportedMeshLevelConversion(from, to) => {
                assert_eq!(from, MeshLevel::X40);
                assert_eq!(to, MeshLevel::Lv1);
            }
            _ => panic!("Expected UnsupportedMeshLevelConversion error"),
        }
    }

    #[test]
    fn test_meshcode_clone_and_copy() {
        let meshcode = MeshCode {
            value: 5339,
            level: MeshLevel::Lv1,
        };
        let cloned = meshcode.clone();
        let copied = meshcode;

        assert_eq!(meshcode, cloned);
        assert_eq!(meshcode, copied);
        assert_eq!(cloned.value, 5339);
        assert_eq!(copied.level, MeshLevel::Lv1);
    }

    #[test]
    fn test_meshcode_equality() {
        let meshcode1 = MeshCode {
            value: 5339,
            level: MeshLevel::Lv1,
        };
        let meshcode2 = MeshCode {
            value: 5339,
            level: MeshLevel::Lv1,
        };
        let meshcode3 = MeshCode {
            value: 5340,
            level: MeshLevel::Lv1,
        };
        let meshcode4 = MeshCode {
            value: 5339,
            level: MeshLevel::Lv2,
        };

        assert_eq!(meshcode1, meshcode2);
        assert_ne!(meshcode1, meshcode3);
        assert_ne!(meshcode1, meshcode4);
    }

    #[test]
    fn test_meshcode_point() {
        // Test cases mirroring the Python test data
        let test_cases = vec![
            // (meshcode, lat_multiplier, lon_multiplier, expected_lat, expected_lon)
            (5339u64, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (53391, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (5339115, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (5339007, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (533900, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (5339006, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (5339001, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (533900617, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (533900116, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (533900005, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (53390000, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (533900001, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (5339000011, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (53390000111, 0.0, 0.0, 35.0 + 1.0 / 3.0, 139.0),
            (53393599212, 0.5, 0.5, 35.6588542, 139.74609375),
        ];

        for (meshcode, lat_multiplier, lon_multiplier, expected_lat, expected_lon) in test_cases {
            let meshcode = MeshCode::try_from(meshcode).unwrap();
            let result = meshcode.point(lat_multiplier, lon_multiplier).unwrap();

            // Check results with approximately equal (7 decimal places)
            assert_relative_eq!(result.0, expected_lat, epsilon = 1e-7);
            assert_relative_eq!(result.1, expected_lon, epsilon = 1e-7);
        }
    }

    #[test]
    fn test_meshcode_contains() {
        let cases = vec![
            // (parent, child, expected)
            (5339, 5339, true),    // Same level
            (5339, 533911, true),  // Child at higher level
            (533900, 5339, false), // Child at lower level
            (5339, 5340, false),   // Same level, disjoint
            (5339, 534001, false), // Child at higher level, disjoint
        ];
        for (parent_value, child_value, expected) in cases {
            let parent = MeshCode::try_from(parent_value).unwrap();
            let child = MeshCode::try_from(child_value).unwrap();
            assert_eq!(
                parent.contains(&child),
                expected,
                "Failed for parent {} and child {}",
                parent_value,
                child_value
            );
        }
    }

    #[test]
    fn test_meshcode_intersects() {
        let cases = vec![
            // (left, right, expected)
            (5339, 5339, true),    // Same level
            (5339, 533911, true),  // right at higher level
            (533900, 5339, true),  // right at lower level
            (5339, 5340, false),   // Same level, disjoint
            (5339, 534001, false), // right at higher level, disjoint
        ];
        for (left_value, right_value, expected) in cases {
            let left = MeshCode::try_from(left_value).unwrap();
            let right = MeshCode::try_from(right_value).unwrap();
            assert_eq!(
                left.intersects(&right),
                expected,
                "Failed for left {} and right {}",
                left_value,
                right_value
            );
        }
    }
}
