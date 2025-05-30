use super::*;
use crate::utils::meshcode::to_meshcode;

/// Generate an envelope of mesh codes that cover the rectangular area
/// defined by the southwest and northeast mesh codes.
///
/// # Arguments
/// * `meshcode_sw` - Southwest mesh code
/// * `meshcode_ne` - Northeast mesh code
///
/// # Returns
/// * `Result<Vec<u64>>` - Vector of mesh codes that cover the area
///
/// # Errors
/// * Returns an error if the mesh levels of the input codes don't match
pub fn to_envelope(meshcode_sw: u64, meshcode_ne: u64) -> Result<Vec<u64>> {
    // Get mesh levels for both codes
    let level_sw = to_meshlevel(&[meshcode_sw])?[0];
    let level_ne = to_meshlevel(&[meshcode_ne])?[0];

    // Check if the mesh levels match
    if level_sw != level_ne {
        return Err(JismeshError::MismatchedMeshLevels(level_sw, level_ne));
    }

    let margin_lat = 0.5;
    let margin_lon = 0.5;

    // Generate mesh points for southwest and northeast corners
    let sw_points = to_meshpoint(&vec![meshcode_sw], &vec![margin_lat], &vec![margin_lon])?;

    let ne_points = to_meshpoint(&vec![meshcode_ne], &vec![1.0], &vec![1.0])?;

    let lat_s = sw_points[0][0];
    let lon_w = sw_points[1][0];
    let lat_n = ne_points[0][0];
    let lon_e = ne_points[1][0];

    make_envelope(lat_s, lon_w, lat_n, lon_e, level_sw)
}

/// Generate mesh codes that intersect with the given mesh code at the specified level.
///
/// # Arguments
/// * `meshcode` - Mesh code to find intersections with
/// * `to_level` - Target mesh level for the intersection
///
/// # Returns
/// * `Result<Vec<u64>>` - Vector of mesh codes that intersect with the input code
pub fn to_intersects(meshcode: u64, to_level: MeshLevel) -> Result<Vec<u64>> {
    // Get mesh level for the input code
    let from_level = to_meshlevel(&[meshcode])?[0];

    let from_unit_lat = unit_lat(from_level);
    let from_unit_lon = unit_lon(from_level);

    let to_unit_lat = unit_lat(to_level);
    let to_unit_lon = unit_lon(to_level);

    // Calculate margins based on the relative unit sizes
    let margin_lat = if to_unit_lat <= from_unit_lat {
        (to_unit_lat / from_unit_lat) / 2.0
    } else {
        0.5
    };

    let margin_lon = if to_unit_lon <= from_unit_lon {
        (to_unit_lon / from_unit_lon) / 2.0
    } else {
        0.5
    };

    // Generate mesh points for the original mesh code
    let from_points_sw = to_meshpoint(&vec![meshcode], &vec![margin_lat], &vec![margin_lon])?;

    let from_points_ne = to_meshpoint(&vec![meshcode], &vec![1.0], &vec![1.0])?;

    let from_lat_s = from_points_sw[0][0];
    let from_lon_w = from_points_sw[1][0];
    let from_lat_n = from_points_ne[0][0];
    let from_lon_e = from_points_ne[1][0];

    make_envelope(from_lat_s, from_lon_w, from_lat_n, from_lon_e, to_level)
}

/// Internal helper function to generate mesh codes within a bounding box
fn make_envelope(
    lat_s: f64,
    lon_w: f64,
    lat_n: f64,
    lon_e: f64,
    level: MeshLevel,
) -> Result<Vec<u64>> {
    let to_unit_lat = unit_lat(level);
    let to_unit_lon = unit_lon(level);

    // Calculate how many meshes we need in each direction
    let lat_count = ((lat_n - lat_s) / to_unit_lat).ceil() as usize;
    let lon_count = ((lon_e - lon_w) / to_unit_lon).ceil() as usize;
    let point_count = lat_count * lon_count;

    let mut lats = Vec::with_capacity(point_count);
    let mut lons = Vec::with_capacity(point_count);
    for i in 0..lat_count {
        let to_lat = lat_s + (i as f64 * to_unit_lat);

        // Generate all longitude points for this latitude
        for j in 0..lon_count {
            let to_lon = lon_w + (j as f64 * to_unit_lon);

            lats.push(to_lat);
            lons.push(to_lon);
        }
    }

    to_meshcode(&lats, &lons, level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_envelope() {
        // Test with level 1 mesh codes for Tokyo area
        let meshcode_sw = 5339; // Southwest corner
        let meshcode_ne = 5339; // Same as SW for simple case

        let result = to_envelope(meshcode_sw, meshcode_ne).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 5339);

        // Test with level 2 mesh codes
        let meshcode_sw = 533900; // Southwest corner
        let meshcode_ne = 533901; // Northeast corner

        let result = to_envelope(meshcode_sw, meshcode_ne).unwrap();
        assert!(result.len() > 1);
        assert!(result.iter().any(|&x| x == 533900));
        assert!(result.iter().any(|&x| x == 533901));
    }

    #[test]
    fn test_to_intersects() {
        // Test conversion from level 1 to level 2
        let meshcode = 5339; // Level 1
        let to_level = MeshLevel::Lv2;

        let result = to_intersects(meshcode, to_level).unwrap();
        assert!(result.len() > 0);

        // All resulting codes should be level 2
        for code in result.iter() {
            let level = to_meshlevel(&[*code]).unwrap();
            assert_eq!(level[0], MeshLevel::Lv2);
        }

        // Test conversion from level 2 to level 3
        let meshcode = 533900; // Level 2
        let to_level = MeshLevel::Lv3;

        let result = to_intersects(meshcode, to_level).unwrap();
        assert!(result.len() > 0);

        // All resulting codes should be level 3
        for code in result.iter() {
            let level = to_meshlevel(&[*code]).unwrap();
            assert_eq!(level[0], MeshLevel::Lv3);
        }
    }

    #[test]
    fn test_error_mismatched_levels() {
        // Test with mismatched mesh levels
        let meshcode_sw = 5339; // Level 1
        let meshcode_ne = 533900; // Level 2

        let result = to_envelope(meshcode_sw, meshcode_ne);
        assert!(result.is_err());
    }
}
