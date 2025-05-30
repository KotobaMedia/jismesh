use super::*;

/// Applies a base level adjustment to latitude and longitude
fn apply_base_adjustment(idx: usize, ab: &[u8], cd: &[u8], lat: &mut [f64], lon: &mut [f64]) {
    lat[idx] = ab[idx] as f64 * UNIT_LAT_LV1;
    lon[idx] = cd[idx] as f64 * UNIT_LON_LV1 + 100.0;
}

/// Applies the level 40000 adjustment to latitude and longitude
fn apply_level_40000(idx: usize, e: &[u8], lat: &mut [f64], lon: &mut [f64]) {
    if e[idx] / 3 == 1 {
        lat[idx] += UNIT_LAT_40000;
    }
    if e[idx] % 2 == 0 {
        lon[idx] += UNIT_LON_40000;
    }
}

/// Applies the level 2 adjustment to latitude and longitude
fn apply_level_2(idx: usize, e: &[u8], f: &[u8], lat: &mut [f64], lon: &mut [f64]) {
    lat[idx] += e[idx] as f64 * UNIT_LAT_LV2;
    lon[idx] += f[idx] as f64 * UNIT_LON_LV2;
}

/// Applies the level 3 adjustment to latitude and longitude
fn apply_level_3(
    idx: usize,
    e: &[u8],
    f: &[u8],
    g: &[u8],
    h: &[u8],
    lat: &mut [f64],
    lon: &mut [f64],
) {
    // First apply level 2 component
    apply_level_2(idx, e, f, lat, lon);

    // Then add level 3 component
    lat[idx] += g[idx] as f64 * UNIT_LAT_LV3;
    lon[idx] += h[idx] as f64 * UNIT_LON_LV3;
}

/// Applies the level 4 adjustment which builds on level 3
fn apply_level_4(
    idx: usize,
    e: &[u8],
    f: &[u8],
    g: &[u8],
    h: &[u8],
    i: &[u8],
    lat: &mut [f64],
    lon: &mut [f64],
) {
    // First apply level 3 component
    apply_level_3(idx, e, f, g, h, lat, lon);

    // Then add level 4 component
    if i[idx] / 3 == 1 {
        lat[idx] += UNIT_LAT_LV4;
    }
    if i[idx] % 2 == 0 {
        lon[idx] += UNIT_LON_LV4;
    }
}

/// Applies the level 5 adjustment which builds on level 4
fn apply_level_5(
    idx: usize,
    e: &[u8],
    f: &[u8],
    g: &[u8],
    h: &[u8],
    i: &[u8],
    j: &[u8],
    lat: &mut [f64],
    lon: &mut [f64],
) {
    // First apply level 4 component
    apply_level_4(idx, e, f, g, h, i, lat, lon);

    // Then add level 5 component
    if j[idx] / 3 == 1 {
        lat[idx] += UNIT_LAT_LV5;
    }
    if j[idx] % 2 == 0 {
        lon[idx] += UNIT_LON_LV5;
    }
}

/// Applies the final multiplier adjustments
fn apply_multipliers(
    idx: usize,
    level: MeshLevel,
    lat_multiplier: &[f64],
    lon_multiplier: &[f64],
    lat: &mut [f64],
    lon: &mut [f64],
) {
    lat[idx] += unit_lat(level) * lat_multiplier[idx.min(lat_multiplier.len() - 1)];
    lon[idx] += unit_lon(level) * lon_multiplier[idx.min(lon_multiplier.len() - 1)];
}

/// Calculates a mesh point (latitude, longitude) from a meshcode and multipliers.
pub fn to_meshpoint(
    meshcode: &[u64],
    lat_multiplier: &[f64],
    lon_multiplier: &[f64],
) -> Result<Vec<Vec<f64>>> {
    // Convert single values to arrays
    let meshcode_len = meshcode.len();

    // Get the mesh level for each code
    let level = to_meshlevel(meshcode)?;

    // Extract parts from meshcode
    let ab = slice(meshcode, 0, 2);
    let cd = slice(meshcode, 2, 4);
    let e = slice(meshcode, 4, 5);
    let f = slice(meshcode, 5, 6);
    let g = slice(meshcode, 6, 7);
    let h = slice(meshcode, 7, 8);
    let i = slice(meshcode, 8, 9);
    let j = slice(meshcode, 9, 10);
    let k = slice(meshcode, 10, 11);

    // Initialize lat and lon vectors
    let mut lat = vec![0.0; meshcode_len];
    let mut lon = vec![0.0; meshcode_len];

    // Process coordinates based on mesh levels
    for idx in 0..meshcode_len {
        // Start with level 1 coordinates (base for all mesh levels)
        apply_base_adjustment(idx, &ab, &cd, &mut lat, &mut lon);

        match level[idx] {
            // Level 1 - already handled in apply_base_adjustment
            MeshLevel::Lv1 => {}

            // Level 40000
            MeshLevel::X40 => {
                apply_level_40000(idx, &e, &mut lat, &mut lon);
            }

            // Level 20000
            MeshLevel::X20 => {
                // Add level 40000 component
                apply_level_40000(idx, &e, &mut lat, &mut lon);

                // Add level 20000 component
                if f[idx] / 3 == 1 {
                    lat[idx] += UNIT_LAT_20000;
                }
                if f[idx] % 2 == 0 {
                    lon[idx] += UNIT_LON_20000;
                }
            }

            // Level 16000
            MeshLevel::X16 => {
                lat[idx] += (e[idx] / 2) as f64 * UNIT_LAT_16000;
                lon[idx] += (f[idx] / 2) as f64 * UNIT_LON_16000;
            }

            // Level 8000
            MeshLevel::X8 => {
                lat[idx] += e[idx] as f64 * UNIT_LAT_8000;
                lon[idx] += f[idx] as f64 * UNIT_LON_8000;
            }

            // Level 4000
            MeshLevel::X4 => {
                // Add level 8000 component
                lat[idx] += e[idx] as f64 * UNIT_LAT_8000;
                lon[idx] += f[idx] as f64 * UNIT_LON_8000;

                // Add level 4000 component
                if h[idx] / 3 == 1 {
                    lat[idx] += UNIT_LAT_4000;
                }
                if h[idx] % 2 == 0 {
                    lon[idx] += UNIT_LON_4000;
                }
            }

            // Level 2
            MeshLevel::Lv2 => {
                apply_level_2(idx, &e, &f, &mut lat, &mut lon);
            }

            // Level 5000
            MeshLevel::X5 => {
                // Add level 2 component
                apply_level_2(idx, &e, &f, &mut lat, &mut lon);

                // Add level 5000 component
                if g[idx] / 3 == 1 {
                    lat[idx] += UNIT_LAT_5000;
                }
                if g[idx] % 2 == 0 {
                    lon[idx] += UNIT_LON_5000;
                }
            }

            // Level 2500
            MeshLevel::X2_5 => {
                // Add level 2 component
                apply_level_2(idx, &e, &f, &mut lat, &mut lon);

                // Add level 5000 component
                if g[idx] / 3 == 1 {
                    lat[idx] += UNIT_LAT_5000;
                }
                if g[idx] % 2 == 0 {
                    lon[idx] += UNIT_LON_5000;
                }

                // Add level 2500 component
                if h[idx] / 3 == 1 {
                    lat[idx] += UNIT_LAT_2500;
                }
                if h[idx] % 2 == 0 {
                    lon[idx] += UNIT_LON_2500;
                }
            }

            // Level 2000
            MeshLevel::X2 => {
                // Add level 2 component
                apply_level_2(idx, &e, &f, &mut lat, &mut lon);

                // Add level 2000 component
                lat[idx] += (g[idx] / 2) as f64 * UNIT_LAT_2000;
                lon[idx] += (h[idx] / 2) as f64 * UNIT_LON_2000;
            }

            // Level 3
            MeshLevel::Lv3 => {
                apply_level_3(idx, &e, &f, &g, &h, &mut lat, &mut lon);
            }

            // Level 4
            MeshLevel::Lv4 => {
                apply_level_4(idx, &e, &f, &g, &h, &i, &mut lat, &mut lon);
            }

            // Level 5
            MeshLevel::Lv5 => {
                apply_level_5(idx, &e, &f, &g, &h, &i, &j, &mut lat, &mut lon);
            }

            // Level 6
            MeshLevel::Lv6 => {
                // First apply level 5 component
                apply_level_5(idx, &e, &f, &g, &h, &i, &j, &mut lat, &mut lon);

                // Then add level 6 component
                if k[idx] / 3 == 1 {
                    lat[idx] += UNIT_LAT_LV6;
                }
                if k[idx] % 2 == 0 {
                    lon[idx] += UNIT_LON_LV6;
                }
            }
        }

        // Add multiplier adjustments
        apply_multipliers(
            idx,
            level[idx],
            lat_multiplier,
            lon_multiplier,
            &mut lat,
            &mut lon,
        );
    }

    // Create a vector of [lat, lon] pairs for each meshcode
    Ok(vec![lat, lon])
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_to_meshpoint() {
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
            // Call the function
            let result = to_meshpoint(&[meshcode], &[lat_multiplier], &[lon_multiplier]).unwrap();

            // Check results with approximately equal (7 decimal places)
            assert_relative_eq!(result[0][0], expected_lat, epsilon = 1e-7);
            assert_relative_eq!(result[1][0], expected_lon, epsilon = 1e-7);
        }
    }

    #[test]
    fn test_to_meshpoint_vector() {
        // Test with vector inputs
        let num_elements = 10;
        let meshcode_value = 53390000111;
        let expected_lat = 35.0 + 1.0 / 3.0;
        let expected_lon = 139.0;

        // Call the function
        let result = to_meshpoint(
            &vec![meshcode_value; num_elements],
            &vec![0.0; num_elements],
            &vec![0.0; num_elements],
        )
        .unwrap();

        // Check results
        for i in 0..num_elements {
            assert_relative_eq!(result[0][i], expected_lat, epsilon = 1e-7);
            assert_relative_eq!(result[1][i], expected_lon, epsilon = 1e-7);
        }
    }
}
