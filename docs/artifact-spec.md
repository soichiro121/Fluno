# Fluno Artifact 仕様

Fluno artifact は、Fluno で扱う workload を配布・検証・実行するための成果物単位です。

このリポジトリでは、artifact のうち **公開 metadata** を扱うための仕様と Rust crate `fluno-artifact-spec` を提供しています。これにより、外部ツールは compiler/runtime の内部実装に依存せず、artifact manifest、ABI、schema、validation profile を読み取れます。

## Artifact manifest

artifact manifest は、artifact の公開 metadata を表す JSON 形式の contract です。主に次の情報を記録します。

- artifact format version
- workload identity
- ABI information
- input / output schema
- target platform
- shape policy
- validation profile
- package hash
- license policy

manifest は artifact の実行本体ではありません。artifact を識別し、検査し、検証するための公開 metadata です。

## ABI と schema

ABI と schema は、artifact が外部とやり取りする境界を表します。

- parameter boundary
- result boundary
- input schema
- output schema
- data shape
- dtype
- validation expectation

runner、validator、auditor は、binary payload や内部 lowering に踏み込まずに、この公開 metadata を扱えます。

## Validation profile

validation profile は、artifact の検証条件を表します。

- tolerance
- digest expectation
- output mode
- publication requirement
- reproducibility に関する制約

Fluno では、artifact を単に生成・実行するだけでなく、あとから検査できる metadata contract を重視しています。

## サンプル manifest

`examples/artifact-manifest/manifest.sample.json` に、公開用の manifest 例を置いています。

このサンプルは metadata の形を示すためのものです。実際の executable artifact を生成するには、Fluno public toolchain の `compile-artifact` を使います。

## この仕様に含まれないもの

この公開仕様は artifact metadata を定義するものです。次のものは含みません。

- artifact を生成する compiler 実装
- artifact builder の内部処理
- artifact runner の内部処理
- binary payload の内部形式
- target-specific lowering
- optimizer
- runtime execution internals


## 関連リファレンス

より体系的な説明は [Artifact Reference](reference/artifact.md) を参照してください。
