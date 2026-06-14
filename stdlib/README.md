# Fluno Standard Library Stable Subset

`stdlib/stable/` には、Fluno 標準ライブラリのうち、Public Technical Preview として公開する安定サブセットを置いています。

これは Fluno 内部標準ライブラリ全体ではありません。外部から API surface を確認しやすくするための公開シグネチャです。

## ファイル

- `core.fln`: core API surface
- `math.fln`: math API surface
- `prob.fln`: probabilistic API surface

## 含まれないもの

次の領域は、runtime behavior や backend strategy と密接に関係するため、現時点の public subset には含めていません。

- neuromorphic API
- stream / reactive runtime API の内部
- accelerator-specific API
- backend-specific intrinsic implementation

## 注意

ここにある `.fln` ファイルは公開シグネチャを確認するためのものです。intrinsic declaration の背後にある compiler/runtime 実装は、このリポジトリには含まれていません。
