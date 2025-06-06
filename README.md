# jismesh-rs

[![Crates.io Version](https://img.shields.io/crates/v/jismesh)](https://crates.io/crates/jismesh)
[![docs.rs](https://img.shields.io/docsrs/jismesh)](https://docs.rs/jismesh/latest/jismesh/)

Utilities for handling and converting JIS X0410 area mesh codes. This Rust port is based on a [Python](https://github.com/hni14/jismesh) implementation.

地域メッシュコードに関するユーティリティです。[Python版](https://github.com/hni14/jismesh)を参照に作成したものです。

## 対応地域メッシュコード
- 1次(標準地域メッシュ 80km四方): 1 : `MeshLevel::Lv1`
- 40倍(拡張統合地域メッシュ 40km四方): 40000 : `MeshLevel::X40`
- 20倍(拡張統合地域メッシュ 20km四方): 20000 : `MeshLevel::X20`
- 16倍(拡張統合地域メッシュ 16km四方): 16000 : `MeshLevel::X16`
- 2次(標準地域メッシュ 10km四方): 2 : `MeshLevel::Lv2`
- 8倍(拡張統合地域メッシュ 8km四方): 8000 : `MeshLevel::X8`
- 5倍(統合地域メッシュ 5km四方): 5000 : `MeshLevel::X5`
- 4倍(拡張統合地域メッシュ 4km四方): 4000 : `MeshLevel::X4`
- 2.5倍(拡張統合地域メッシュ 2.5km四方): 2500 : `MeshLevel::X2_5`
- 2倍(統合地域メッシュ 2km四方): 2000 : `MeshLevel::X2`
- 3次(標準地域メッシュ 1km四方): 3 : `MeshLevel::Lv3`
- 4次(分割地域メッシュ 500m四方): 4 : `MeshLevel::Lv4`
- 5次(分割地域メッシュ 250m四方): 5 : `MeshLevel::Lv5`
- 6次(分割地域メッシュ 125m四方): 6 : `MeshLevel::Lv6`

## インストール

```bash
cargo add jismesh
```

## 利用

**注意: このライブラリは [Python](https://github.com/hni14/jismesh) 版と同様に、「緯度」「軽度」の順で引数を受け付けています**

v0.3.0 からインターフェースが変わりました。今まで `u64` でメッシュコードを表していましたが、現在は `MeshCode` に変わっています。`u64 -> MeshCode` は TryFrom 、 `MeshCode -> u64` は From の impl あるので、変換に使ってください。今後、処理等は全部 `MeshCode` に移行していく予定です。使用例は下記参照してください。

### 緯度軽度（世界測地系）からメッシュコードを生成する場合

```rust
use jismesh::{MeshCode, MeshLevel::Lv3, to_meshcode};

// MeshCode を使う場合
let meshcode = MeshCode::try_from_latlng(35.658581, 139.745433, Lv3).unwrap();
// MeshCode に MeshLevel が含まれている
assert_eq!(meshcode.level, Lv3);
// u64 と impl PartialEq が実装されています
assert_eq!(meshcode, 53393599);
// u64 として必要なときは From / Into 使えます
let meshcode_u64: u64 = meshcode.into();
assert_eq!(meshcode_u64, 53393599);

// Python 版インターフェース
let codes = to_meshcode(&[35.658581], &[139.745433], Lv3).unwrap();
assert_eq!(codes, &[53393599]);

// 複数点を計算する場合
let codes = to_meshcode(
    &[35.658581, 34.987574],
    &[139.745433, 135.759363],
    Lv3,
).unwrap();
assert_eq!(codes, &[53393599, 52353680]);
```

### 地域メッシュコードから次数を計算する場合

```rust
use jismesh::{MeshCode, MeshLevel::Lv3, to_meshlevel};

let meshcode = MeshCode::try_from(53393599u64).unwrap();
assert_eq!(meshcode.level, Lv3);

// Python 版インターフェース
let levels = to_meshlevel(&[53393599, 52353680]).unwrap();
assert_eq!(levels, &[Lv3, Lv3]);
```

### 地域メッシュコードから緯度経度を計算する場合

```rust
use jismesh::{MeshCode, to_meshpoint};

let code = MeshCode::try_from(53393599u64).unwrap();
// 南西端の緯度経度を求める。
assert_eq!(
    code.point(0.0, 0.0).unwrap(),
    (35.65833333333333, 139.7375)
);
// 北東端の緯度経度を求める。
assert_eq!(
    code.point(1.0, 1.0).unwrap(),
    (35.666666666666664, 139.75)
);
// 中心点の緯度経度を求める。
assert_eq!(
    code.point(0.5, 0.5).unwrap(),
    (35.6625, 139.74375)
);

// Python 版インターフェース
let points = to_meshpoint(&[53393599], &[0.0], &[0.0]).unwrap();
assert_eq!(points, &[&[35.65833333333333],&[139.7375]]);

let points = to_meshpoint(&[53393599], &[1.0], &[1.0]).unwrap();
assert_eq!(points, &[&[35.666666666666664],&[139.75]]);

let points = to_meshpoint(&[53393599], &[0.5], &[0.5]).unwrap();
assert_eq!(points, &[&[35.6625],&[139.74375]]);
```

### 次数から `MeshLevel` の変換

```rust
use jismesh::MeshLevel;

let lv = MeshLevel::try_from(3).unwrap();
assert_eq!(lv, MeshLevel::Lv3);
```
