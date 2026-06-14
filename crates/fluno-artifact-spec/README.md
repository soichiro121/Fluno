# fluno-artifact-spec

`fluno-artifact-spec` は、Fluno artifact metadata を扱うための Rust crate です。

artifact manifest、ABI、input/output schema、validation profile、hash utility を公開データ型として提供します。

## 使い方

```toml
[dependencies]
fluno-artifact-spec = { git = "https://github.com/soichiro121/Fluno", package = "fluno-artifact-spec" }
```

```rust
use fluno_artifact_spec::ArtifactManifest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("manifest.json")?;
    let manifest: ArtifactManifest = serde_json::from_str(&json)?;
    println!("{manifest:#?}");
    Ok(())
}
```

## 提供するもの

- `ArtifactManifest`
- ABI signature
- input / output schema
- validation profile
- package file manifest
- manifest hash utility

## 提供しないもの

この crate は artifact metadata の仕様を提供するものです。次の実装は含みません。

- artifact builder
- artifact runtime
- compiler
- target backend implementation
- binary payload の内部形式

## ライセンス

`Apache-2.0 OR MIT`
