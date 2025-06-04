use super::{JismeshError, Result};
use std::fmt;
use strum_macros::{EnumIter, EnumString};

/// 地域メッシュコードの次数
#[derive(Debug, Clone, Copy, Eq, EnumIter, EnumString, PartialEq, PartialOrd, Ord, Hash)]
pub enum MeshLevel {
    /// 1次(80km四方) 4桁
    Lv1 = 1,
    /// 40倍(40km四方)
    X40 = 40000,
    /// 20倍(20km四方)
    X20 = 20000,
    /// 16倍(16km四方)
    X16 = 16000,
    /// 2次(10km四方) 6桁
    Lv2 = 2,
    /// 8倍(8km四方)
    X8 = 8000,
    /// 5倍(5km四方)
    X5 = 5000,
    /// 4倍(4km四方)
    X4 = 4000,
    /// 2.5倍(2.5km四方)
    X2_5 = 2500,
    /// 2倍(2km四方)
    X2 = 2000,
    /// 3次(1km四方) 8桁
    Lv3 = 3,
    /// 4次(500m四方)
    Lv4 = 4,
    /// 5次(250m四方)
    Lv5 = 5,
    /// 6次(125m四方)
    Lv6 = 6,
}

impl MeshLevel {
    /// メッシュコードの日本語名を取得する
    pub fn to_string_jp(&self) -> &str {
        match self {
            MeshLevel::Lv1 => "1次",
            MeshLevel::X40 => "40倍",
            MeshLevel::X20 => "20倍",
            MeshLevel::X16 => "16倍",
            MeshLevel::Lv2 => "2次",
            MeshLevel::X8 => "8倍",
            MeshLevel::X5 => "5倍",
            MeshLevel::X4 => "4倍",
            MeshLevel::X2_5 => "2.5倍",
            MeshLevel::X2 => "2倍",
            MeshLevel::Lv3 => "3次",
            MeshLevel::Lv4 => "4次",
            MeshLevel::Lv5 => "5次",
            MeshLevel::Lv6 => "6次",
        }
    }
    /// メッシュコードのおおよそのサイズを取得する（日本語）
    /// 例: "80km四方"
    pub fn to_size_jp(&self) -> &str {
        match self {
            MeshLevel::Lv1 => "80km四方",
            MeshLevel::X40 => "40km四方",
            MeshLevel::X20 => "20km四方",
            MeshLevel::X16 => "16km四方",
            MeshLevel::Lv2 => "10km四方",
            MeshLevel::X8 => "8km四方",
            MeshLevel::X5 => "5km四方",
            MeshLevel::X4 => "4km四方",
            MeshLevel::X2_5 => "2.5km四方",
            MeshLevel::X2 => "2km四方",
            MeshLevel::Lv3 => "1km四方",
            MeshLevel::Lv4 => "500m四方",
            MeshLevel::Lv5 => "250m四方",
            MeshLevel::Lv6 => "125m四方",
        }
    }
}

impl fmt::Display for MeshLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<usize> for MeshLevel {
    type Error = JismeshError;

    fn try_from(value: usize) -> Result<Self> {
        match value {
            1 => Ok(MeshLevel::Lv1),
            40000 => Ok(MeshLevel::X40),
            20000 => Ok(MeshLevel::X20),
            16000 => Ok(MeshLevel::X16),
            2 => Ok(MeshLevel::Lv2),
            8000 => Ok(MeshLevel::X8),
            5000 => Ok(MeshLevel::X5),
            4000 => Ok(MeshLevel::X4),
            2500 => Ok(MeshLevel::X2_5),
            2000 => Ok(MeshLevel::X2),
            3 => Ok(MeshLevel::Lv3),
            4 => Ok(MeshLevel::Lv4),
            5 => Ok(MeshLevel::Lv5),
            6 => Ok(MeshLevel::Lv6),
            _ => Err(JismeshError::InvalidMeshLevel(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn tesh_meshlevel_sort() {
        let mut levels = vec![
            MeshLevel::Lv6,
            MeshLevel::Lv3,
            MeshLevel::Lv2,
            MeshLevel::Lv5,
            MeshLevel::Lv4,
            MeshLevel::Lv1,
        ];
        levels.sort();
        assert_eq!(
            levels,
            vec![
                MeshLevel::Lv1,
                MeshLevel::Lv2,
                MeshLevel::Lv3,
                MeshLevel::Lv4,
                MeshLevel::Lv5,
                MeshLevel::Lv6
            ]
        );
    }

    #[test]
    fn test_meshlevel_conversion() {
        assert_eq!(MeshLevel::try_from(1).unwrap(), MeshLevel::Lv1);
        assert_eq!(MeshLevel::try_from(6).unwrap(), MeshLevel::Lv6);

        // Test invalid conversion
        let result = MeshLevel::try_from(9999);
        assert!(result.is_err());
    }

    #[test]
    fn test_meshlevel_enum_iter() {
        let levels: Vec<MeshLevel> = MeshLevel::iter().collect();
        assert_eq!(levels.len(), 14);
        assert_eq!(levels[0], MeshLevel::Lv1);
        assert_eq!(levels[13], MeshLevel::Lv6);
    }

    #[test]
    fn test_meshlevel_string() {
        let level: MeshLevel = "Lv1".parse().unwrap();
        assert_eq!(level, MeshLevel::Lv1);
        assert_eq!(level.to_string(), "Lv1");

        let level: MeshLevel = "Lv6".parse().unwrap();
        assert_eq!(level, MeshLevel::Lv6);
        assert_eq!(level.to_string(), "Lv6");

        // Test invalid string
        let result: Result<_> = "Invalid".parse::<MeshLevel>().map_err(|e| e.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_to_jp_str() {
        let level = MeshLevel::Lv1;
        assert_eq!(level.to_string_jp(), "1次");
        assert_eq!(level.to_size_jp(), "80km四方");
        let level = MeshLevel::Lv6;
        assert_eq!(level.to_string_jp(), "6次");
        assert_eq!(level.to_size_jp(), "125m四方");
        let level = MeshLevel::X40;
        assert_eq!(level.to_string_jp(), "40倍");
        assert_eq!(level.to_size_jp(), "40km四方");
    }
}
