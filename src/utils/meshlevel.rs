use super::*;
use crate::utils::error::JismeshError;
use ndarray::Array1;

/// Determines the mesh level from a meshcode.
pub fn to_meshlevel(meshcode: &Array1<u64>) -> Result<Vec<MeshLevel>> {
    // Check if any value is 0 or invalid
    if meshcode.iter().any(|&code| code == 0) {
        return Err(JismeshError::UnknownMeshLevelForCode(0));
    }

    // Calculate number of digits for each meshcode
    let num_digits = meshcode.mapv(|code| (code as f64).log10().floor() as usize + 1);

    // Extract the g and i digits needed for determining mesh levels
    let g = slice(&meshcode, 6, 7);
    let i = slice(&meshcode, 8, 9);
    let j = slice(&meshcode, 9, 10);
    let k = slice(&meshcode, 10, 11);

    // Create a result vector to store mesh levels
    let mut results = Vec::with_capacity(meshcode.len());

    // Determine mesh level for each meshcode
    for idx in 0..meshcode.len() {
        let level = match num_digits[idx] {
            4 => MeshLevel::Lv1,
            5 => MeshLevel::X40,
            6 => MeshLevel::Lv2,
            7 => match g[idx] {
                1..=4 => MeshLevel::X5,
                5 => MeshLevel::X20,
                6 => MeshLevel::X8,
                7 => MeshLevel::X16,
                _ => return Err(JismeshError::InvalidMeshcodeAtLevel(7, meshcode[idx])),
            },
            8 => MeshLevel::Lv3,
            9 => match i[idx] {
                1..=4 => MeshLevel::Lv4,
                5 => MeshLevel::X2,
                6 => MeshLevel::X2_5,
                7 => MeshLevel::X4,
                _ => return Err(JismeshError::InvalidMeshcodeAtLevel(9, meshcode[idx])),
            },
            10 => match j[idx] {
                1..=4 => MeshLevel::Lv5,
                _ => return Err(JismeshError::InvalidMeshcodeAtLevel(10, meshcode[idx])),
            },
            11 => match k[idx] {
                1..=4 => MeshLevel::Lv6,
                _ => return Err(JismeshError::InvalidMeshcodeAtLevel(11, meshcode[idx])),
            },
            _ => return Err(JismeshError::UnknownMeshLevelForCode(meshcode[idx])),
        };

        results.push(level);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_meshlevel() {
        let test_cases = vec![
            (5339, MeshLevel::Lv1),
            (53392, MeshLevel::X40),
            (5339235, MeshLevel::X20),
            (5339467, MeshLevel::X16),
            (533935, MeshLevel::Lv2),
            (5339476, MeshLevel::X8),
            (5339354, MeshLevel::X5),
            (533947637, MeshLevel::X4),
            (533935446, MeshLevel::X2_5),
            (533935885, MeshLevel::X2),
            (53393599, MeshLevel::Lv3),
            (533935992, MeshLevel::Lv4),
            (5339359921, MeshLevel::Lv5),
            (53393599212, MeshLevel::Lv6),
            (5235, MeshLevel::Lv1),
            (52352, MeshLevel::X40),
            (5235245, MeshLevel::X20),
            (5235467, MeshLevel::X16),
            (523536, MeshLevel::Lv2),
            (5235476, MeshLevel::X8),
            (5235363, MeshLevel::X5),
            (523547647, MeshLevel::X4),
            (523536336, MeshLevel::X2_5),
            (523536805, MeshLevel::X2),
            (52353680, MeshLevel::Lv3),
            (523536804, MeshLevel::Lv4),
            (5235368041, MeshLevel::Lv5),
            (52353680412, MeshLevel::Lv6),
        ];
        for (meshcode, expected) in test_cases {
            assert_eq!(
                to_meshlevel(&array![meshcode]),
                Ok(vec![expected]),
                "Failed for meshcode: {}",
                meshcode
            );
        }
    }

    #[test]
    fn test_meshlevel_invalid() {
        let res = to_meshlevel(&array![5]);
        assert!(res.is_err());
    }
}
