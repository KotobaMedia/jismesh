use super::*;
use crate::utils::error::JismeshError;
use ndarray::Array1;

/// Converts latitude & longitude to a meshcode.
/// 緯度経度から指定次の地域メッシュコードを算出する。
///
/// Args:
/// * lat: 世界測地系の緯度(度単位)
/// * lon: 世界測地系の経度(度単位)
pub fn to_meshcode(lat: &Array1<f64>, lon: &Array1<f64>, level: MeshLevel) -> Result<Array1<u64>> {
    // Validate bounds for all values in the arrays
    for &lat_val in lat.iter() {
        if !(0.0 <= lat_val && lat_val < 66.66) {
            return Err(JismeshError::LatitudeOutOfBounds(lat_val));
        }
    }

    for &lon_val in lon.iter() {
        if !(100.0 <= lon_val && lon_val < 180.0) {
            return Err(JismeshError::LongitudeOutOfBounds(lon_val));
        }
    }

    // Create output array
    let mut result = Array1::zeros(lat.len().max(lon.len()));

    for i in 0..result.len() {
        let lat_val = lat[i % lat.len()];
        let lon_val = lon[i % lon.len()];

        // Calculate mesh code based on level
        result[i] = match level {
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
    }

    Ok(result)
}

// Helper functions for calculating meshcodes at various levels
fn meshcode_lv1(lat: f64, lon: f64) -> u64 {
    let rem_lat_lv0 = lat;
    let rem_lon_lv0 = lon % 100.0;
    let ab = (rem_lat_lv0 / UNIT_LAT_LV1) as u64;
    let cd = (rem_lon_lv0 / UNIT_LON_LV1) as u64;
    ab * 100 + cd
}

fn meshcode_40000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_40000) as u64 * 2 + (rem_lon_lv1 / UNIT_LON_40000) as u64 + 1;
    base * 10 + e
}

fn meshcode_20000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_40000(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let rem_lat_40000 = rem_lat_lv1 % UNIT_LAT_40000;
    let rem_lon_40000 = rem_lon_lv1 % UNIT_LON_40000;
    let f =
        (rem_lat_40000 / UNIT_LAT_20000) as u64 * 2 + (rem_lon_40000 / UNIT_LON_20000) as u64 + 1;
    let g = 5;
    base * 100 + f * 10 + g
}

fn meshcode_16000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_16000) as u64 * 2;
    let f = (rem_lon_lv1 / UNIT_LON_16000) as u64 * 2;
    let g = 7;
    base * 1000 + e * 100 + f * 10 + g
}

fn meshcode_lv2(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_LV2) as u64;
    let f = (rem_lon_lv1 / UNIT_LON_LV2) as u64;
    base * 100 + e * 10 + f
}

fn meshcode_8000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv1(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let e = (rem_lat_lv1 / UNIT_LAT_8000) as u64;
    let f = (rem_lon_lv1 / UNIT_LON_8000) as u64;
    let g = 6;
    base * 1000 + e * 100 + f * 10 + g
}

fn meshcode_5000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv2(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let g = (rem_lat_lv2 / UNIT_LAT_5000) as u64 * 2 + (rem_lon_lv2 / UNIT_LON_5000) as u64 + 1;
    base * 10 + g
}

fn meshcode_4000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_8000(lat, lon);
    let rem_lat_lv1 = lat % UNIT_LAT_LV1;
    let rem_lon_lv1 = lon % 100.0 % UNIT_LON_LV1;
    let rem_lat_8000 = rem_lat_lv1 % UNIT_LAT_8000;
    let rem_lon_8000 = rem_lon_lv1 % UNIT_LON_8000;
    let h = (rem_lat_8000 / UNIT_LAT_4000) as u64 * 2 + (rem_lon_8000 / UNIT_LON_4000) as u64 + 1;
    let i = 7;
    base * 100 + h * 10 + i
}

fn meshcode_2500(lat: f64, lon: f64) -> u64 {
    let base = meshcode_5000(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let rem_lat_5000 = rem_lat_lv2 % UNIT_LAT_5000;
    let rem_lon_5000 = rem_lon_lv2 % UNIT_LON_5000;
    let h = (rem_lat_5000 / UNIT_LAT_2500) as u64 * 2 + (rem_lon_5000 / UNIT_LON_2500) as u64 + 1;
    let i = 6;
    base * 100 + h * 10 + i
}

fn meshcode_2000(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv2(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let g = (rem_lat_lv2 / UNIT_LAT_2000) as u64 * 2;
    let h = (rem_lon_lv2 / UNIT_LON_2000) as u64 * 2;
    let i = 5;
    base * 1000 + g * 100 + h * 10 + i
}

fn meshcode_lv3(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv2(lat, lon);
    let rem_lat_lv2 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2;
    let rem_lon_lv2 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2;
    let g = (rem_lat_lv2 / UNIT_LAT_LV3) as u64;
    let h = (rem_lon_lv2 / UNIT_LON_LV3) as u64;
    base * 100 + g * 10 + h
}

fn meshcode_lv4(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv3(lat, lon);
    let rem_lat_lv3 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2 % UNIT_LAT_LV3;
    let rem_lon_lv3 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2 % UNIT_LON_LV3;
    let i = (rem_lat_lv3 / UNIT_LAT_LV4) as u64 * 2 + (rem_lon_lv3 / UNIT_LON_LV4) as u64 + 1;
    base * 10 + i
}

fn meshcode_lv5(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv4(lat, lon);
    let rem_lat_lv4 = lat % UNIT_LAT_LV1 % UNIT_LAT_LV2 % UNIT_LAT_LV3 % UNIT_LAT_LV4;
    let rem_lon_lv4 = lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2 % UNIT_LON_LV3 % UNIT_LON_LV4;
    let j = (rem_lat_lv4 / UNIT_LAT_LV5) as u64 * 2 + (rem_lon_lv4 / UNIT_LON_LV5) as u64 + 1;
    base * 10 + j
}

fn meshcode_lv6(lat: f64, lon: f64) -> u64 {
    let base = meshcode_lv5(lat, lon);
    let rem_lat_lv5 =
        lat % UNIT_LAT_LV1 % UNIT_LAT_LV2 % UNIT_LAT_LV3 % UNIT_LAT_LV4 % UNIT_LAT_LV5;
    let rem_lon_lv5 =
        lon % 100.0 % UNIT_LON_LV1 % UNIT_LON_LV2 % UNIT_LON_LV3 % UNIT_LON_LV4 % UNIT_LON_LV5;
    let k = (rem_lat_lv5 / UNIT_LAT_LV6) as u64 * 2 + (rem_lon_lv5 / UNIT_LON_LV6) as u64 + 1;
    base * 10 + k
}

#[cfg(test)]
mod tests {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_error_invalid_latitude_min() {
        let res = to_meshcode(&array![-0.1], &array![139.745433], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_error_invalid_latitude_max() {
        let res = to_meshcode(&array![66.66], &array![139.745433], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_error_invalid_longitude_min() {
        let res = to_meshcode(&array![35.658581], &array![99.99], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_error_invalid_longitude_max() {
        let res = to_meshcode(&array![35.658581], &array![180.0], MeshLevel::Lv1);
        assert!(res.is_err());
    }

    #[test]
    fn test_tokyo_meshcodes() {
        let lat = array![35.658581];
        let lon = array![139.745433];
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
            let result = to_meshcode(&lat, &lon, level);
            assert_eq!(result, Ok(array![expected]), "Failed for level {:?}", level);
        }
    }

    #[test]
    fn test_kyoto_meshcodes() {
        let lat = array![34.987574];
        let lon = array![135.759363];
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
            let result = to_meshcode(&lat, &lon, level);
            assert_eq!(result, Ok(array![expected]), "Failed for level {:?}", level);
        }
    }
}
