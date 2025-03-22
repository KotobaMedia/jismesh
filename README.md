# jismesh-rs

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

### 緯度軽度（世界測地系）からメッシュコードを生成する場合

```rust
use ndarray::array;
use jismesh::{MeshLevel, to_meshcode};

let codes = to_meshcode(&array![35.658581], &array![139.745433], MeshLevel::Lv3).unwrap();
assert_eq!(codes, array![53393599]);

// 複数点を計算する場合
let codes = to_meshcode(
    &array![35.658581, 34.987574],
    &array![139.745433, 135.759363],
    MeshLevel::Lv3,
).unwrap();
assert_eq!(codes, array![53393599, 52353680]);
```

### 地域メッシュコードから次数を計算する場合

```rust
use ndarray::array;
use jismesh::{MeshLevel::Lv3, to_meshlevel};

let levels = to_meshlevel(&array![53393599, 52353680]).unwrap();
assert_eq!(levels, vec![Lv3, Lv3]);
```

### 地域メッシュコードから緯度経度を計算する場合

```rust
use ndarray::array;
use jismesh::{MeshLevel::Lv3, to_meshpoint};

// 南西端の緯度経度を求める。
let points = to_meshpoint(array![53393599], array![0.0], array![0.0]).unwrap();
assert_eq!(points, array![[35.65833333333333],[139.7375]]);

// 北東端の緯度経度を求める。
let points = to_meshpoint(array![53393599], array![1.0], array![1.0]).unwrap();
assert_eq!(points, array![[35.666666666666664],[139.75]]);

// 中心点の緯度経度を求める。
let points = to_meshpoint(array![53393599], array![0.5], array![0.5]).unwrap();
assert_eq!(points, array![[35.6625],[139.74375]]);
```

### 次数から `MeshLevel` の変換

```rust
use jismesh::MeshLevel;

let lv = MeshLevel::try_from(3).unwrap();
assert_eq!(lv, MeshLevel::Lv3);
```
