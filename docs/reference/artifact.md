# Artifact Reference

この文書は、Fluno artifact の公開概念と、`fluno-artifact-spec` crate が扱う metadata の範囲を説明します。

Fluno artifact は、`.fln` source から生成される配布・検証・実行の単位です。artifact は単なる実行ファイルではなく、外部から検査できるmanifestと、runtimeが扱うpayloadを含む成果物として設計されています。

## なぜartifactを使うのか

Flunoでは、sourceを直接実行するだけでなく、次の性質を持つ成果物としてworkloadを扱うことを重視しています。

- 何を実行するartifactかを識別できる。
- 入出力schemaやABIを検査できる。
- targetやvalidation条件をmetadataとして持てる。
- compiler/runtimeの内部IRに依存せず、外部toolingがartifactを扱える。
- 将来のbackendやrunnerが増えても、公開境界をmanifest側に置ける。

## artifact生成の流れ

```text
source.fln
  ↓ fluno check
semantic validation
  ↓ fluno compile-artifact
artifact directory
  ├─ manifest.json
  └─ payload / metadata files
```

Public Technical Preview では、artifactの内部payload形式は公開APIではありません。外部toolingが依存すべきなのは、公開manifestと `fluno-artifact-spec` が扱うmetadataです。

## `manifest.json`

artifact manifest は、artifact の公開metadataを表すJSONファイルです。

主な役割:

- artifact のformat versionを示す。
- workload identityを示す。
- ABI情報を示す。
- input / output schemaを示す。
- target platformを示す。
- validation profileを示す。
- hash / digest 情報を示す。
- license policyや配布条件を示す。

サンプルは `examples/artifact-manifest/manifest.sample.json` にあります。

## ABI

ABI は、artifact が外部とやり取りする境界です。

Fluno artifactでは、次のような情報をmetadataとして扱います。

- parameter boundary
- result boundary
- type information
- shape policy
- calling conventionに相当する公開情報

実際のruntime呼び出し規約や内部payload形式は、Public Technical Previewでは公開対象外です。

## Schema

Schema は、artifactの入力・出力データの構造を表します。

例:

- scalar value
- structured value
- tensor-like shape metadata
- dtype
- validation expectation

公開artifact toolingは、schemaを使ってartifactの外部境界を検査できます。

## Validation profile

Validation profile は、artifact を検証する条件を表します。

例:

- 許容誤差
- digest expectation
- reproducibilityに関する条件
- publication requirement
- output mode

Flunoでは、artifactを生成して終わりではなく、あとから検査できるmetadata contractを重視しています。

## `fluno-artifact-spec` crate

`fluno-artifact-spec` は、artifact manifest、ABI、schema、validation profile、hash manifestなどの公開データ構造を提供します。

このcrateは、artifact metadata toolingや外部検証ツールの基盤として使うことを想定しています。

```toml
[dependencies]
fluno-artifact-spec = { git = "https://github.com/soichiro121/Fluno", package = "fluno-artifact-spec" }
```

## 公開APIとして依存してよいもの

Public Technical Previewで外部toolingが依存してよいのは、基本的に次の範囲です。

- `manifest.json` の公開metadata
- `fluno-artifact-spec` の公開型
- `fluno artifact inspect` の出力
- `fluno artifact validate` の結果

## 依存すべきでないもの

次のものは公開APIではありません。

- artifact内部payloadの物理形式
- compiler内部IR
- Bytecode形式
- MLIR lowering結果
- optimizerの中間状態
- runner内部実装
- target-specific backendの詳細

これらは将来のバージョンで変更される可能性があります。
