# Current Limitations

この文書は、Fluno Public Technical Preview の現在の制限をまとめたものです。

Fluno は開発中の実験的言語・toolchainです。公開している範囲は、外部から試しやすくするためのpublic surfaceであり、完全なproduction-ready releaseではありません。

## ステータス

```text
Status: Public Technical Preview
Production use: not recommended
Binary signing: not available yet
Compiler/runtime core source: not included
```

API、構文、artifact形式、toolchain command、JSON出力は今後変更される可能性があります。

## compiler/runtime core は含まれていない

この公開リポジトリには、次の実装は含まれていません。

- typechecker / semantic checker
- compiler pipeline
- Core IR lowering
- Bytecode VM
- runtime implementation
- MLIR backend
- optimizer
- artifact builder / runner の詳細
- tensor execution の詳細
- accelerator / neuromorphic backend の詳細
- 非公開 benchmark / fixture
- development-only CLI

公開しているのは、syntax crate、artifact spec crate、stable stdlib subset、examples、docsです。

## binaryはTechnical Preview

`.fln` ファイルを実際にcheck / compile-artifact / runするには、Releaseで配布しているlimited public toolchain binaryを使います。

現在のbinaryはTechnical Previewです。本番環境での利用はまだ推奨していません。

配布bundleには、確認用の次のファイルを含めています。

```text
SHA256SUMS.txt
PUBLIC_RELEASE_MANIFEST.json
```

## 未署名binary

現時点のbinaryは未署名です。

そのため、OSや配布経路によっては次のような挙動になる場合があります。

- WindowsでSmartScreenやDefenderの警告が出る。
- macOSでGatekeeperの警告が出る。
- ダウンロード後に実行許可の手動設定が必要になる。

これは、Technical Previewとしての制限です。

## 公開toolchainのcommand範囲

公開しているcommandは限定されています。

```text
fluno version
fluno check
fluno compile-artifact
fluno artifact inspect
fluno artifact validate
fluno artifact run
```

次のcommandや機能は含めていません。

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

## 標準ライブラリはサブセット

`stdlib/stable` は、Fluno standard library の公開サブセットです。

含まれるもの:

```text
core.fln
math.fln
prob.fln
```

含まれないもの:

- full runtime stdlib
- stream / reactive runtime の詳細
- tensor runtime の詳細
- AD execution の詳細
- hardware backend specific APIs

## language surface は変更される可能性がある

Public Technical Preview では、構文や型規則が変更される可能性があります。

特に、次の領域は今後変更される可能性があります。

- complex pattern matching
- module/import resolution
- generic / trait / impl semantics
- effect / async / stream / reactive syntax
- artifact format version
- public JSON output schema


## ドメイン系構文の扱い

Fluno には、確率分布、`Signal<T>`、`Event<T>`、`stream { ... }`、`Future<T>`、`spawn`、`.await`、`EstimatorKind`、`SurrogateRule` など、AI・リアクティブ処理・非同期実行・AD系のためのsyntax surfaceが含まれています。

これらは Fluno の方向性を示す重要な公開surfaceです。ただし、Public Technical Preview では、各ドメイン機能のruntime semanticsやbackend loweringがすべて安定公開されているわけではありません。安定度は [Domain Syntax Reference](domain-syntax.md) を参照してください。

## 推奨される使い方

現時点で推奨する使い方は次の通りです。

- READMEとexamplesに沿ってtoolchainを試す。
- `fluno-syntax` を構文解析・editor tooling・外部検査に使う。
- `fluno-artifact-spec` をartifact metadata toolingに使う。
- `stdlib/stable` の公開シグネチャを参照する。
- Zenn記事やREADMEからFlunoの設計方針を理解する。

本番サービス、商用運用、長期互換性が必要なシステムでの利用は、安定版まで待つことを推奨します。
