# Fluno Public Technical Preview リファレンス

このディレクトリには、Fluno Public Technical Preview で公開している範囲の実用リファレンスを置いています。

Fluno はまだ安定版ではありません。ここで説明する内容は、現在公開している `fluno-syntax`、`fluno-artifact-spec`、`stdlib/stable`、および limited public toolchain binary を基準にしたものです。

## 目次

- [Language Reference](language.md)
- [Domain Syntax Reference](domain-syntax.md)
- [Standard Library Reference](stdlib.md)
- [Toolchain Reference](toolchain.md)
- [Artifact Reference](artifact.md)
- [Current Limitations](limitations.md)

## このリファレンスの位置づけ

このリファレンスは、Fluno の完全な内部仕様書ではありません。compiler/runtime core、typechecker、VM、MLIR backend、optimizer の内部設計は、この公開リポジトリの対象外です。

公開しているのは、Fluno を外部から試し、構文・ドメイン系syntax surface・artifact metadata・標準ライブラリ surface を理解するために必要な範囲です。
