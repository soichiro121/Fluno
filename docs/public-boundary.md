# 公開範囲

このリポジトリは、Fluno の Public Technical Preview として、外部から確認しやすい部分を公開するためのものです。

Fluno のすべての実装を含むリポジトリではありません。構文、artifact metadata、標準ライブラリの公開シグネチャ、サンプル、ドキュメントを公開し、compiler/runtime の中核実装は現時点では含めていません。

## 公開しているもの

```text
crates/fluno-syntax
  Lexer / parser / AST / diagnostics / source map

crates/fluno-artifact-spec
  Artifact manifest / ABI / schema / validation profile / hash utility

stdlib/stable
  core / math / prob の公開サブセット

docs
  構文概要、artifact 仕様、公開範囲、配布方針

examples
  最小例、代表的構文例、manifest 例
```

## 公開していないもの

次の実装は、このリポジトリには含まれていません。

- typechecker / semantic checker
- compiler pipeline
- Core IR lowering
- Bytecode VM
- runtime 実装
- MLIR backend
- optimizer
- artifact builder / runner の内部実装
- tensor execution の詳細
- accelerator / neuromorphic backend
- 非公開 benchmark / fixture
- diagnostic dump / trace command

## Binary toolchain との関係

`.fln` ファイルを実際に `check` / `compile-artifact` / `run` するには、GitHub Releases で配布する limited public toolchain binary を使います。

公開している主な command:

```text
fluno version
fluno check
fluno compile-artifact
fluno artifact inspect
fluno artifact validate
fluno artifact run
```

公開していない command:

```text
REPL
raw compile
Core IR dump
bytecode dump
MLIR dump
lowering trace
optimizer trace
non-public benchmark command
```

## なぜ境界を分けているのか

Fluno は、構文、型検査、runtime、artifact、backend、optimizer が密接に結びついた処理系です。初期段階で未安定な内部 API まで公開すると、変更しにくい外部 contract のように見えてしまいます。

そのため、まずは外部から検査・試用しやすい部分を公開し、compiler/runtime core は安定性・ライセンス・配布条件を整理しながら段階的に扱います。


## 制限事項

現在の制限は [Current Limitations](reference/limitations.md) を参照してください。
