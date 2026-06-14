# Artifact Manifest サンプル

`manifest.sample.json` は、Fluno artifact manifest の公開サンプルです。

このサンプルでは、artifact にどのような metadata が含まれるかを確認できます。

## 含まれる情報

- artifact format version
- workload identity
- ABI information
- input / output schema
- validation profile
- package hash
- license policy

## 注意

この manifest はドキュメント用のサンプルです。実際の executable artifact を生成するものではありません。

実際に artifact を生成する場合は、Fluno public toolchain の `compile-artifact` を使います。
