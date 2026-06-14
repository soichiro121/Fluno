# 配布方針

Fluno は現在、Public Technical Preview として公開しています。

この段階では、構文・artifact 仕様・サンプル・公開標準ライブラリを OSS として公開し、`.fln` ファイルを試すための limited public toolchain binary を別途配布します。

## 公開形態

```text
Public OSS repository
  syntax crate
  artifact spec crate
  stdlib stable subset
  docs
  examples

Public toolchain binary
  check
  compile-artifact
  artifact inspect
  artifact validate
  artifact run

Not currently open source
  compiler core
  runtime
  VM
  MLIR lowering
  optimizer
```

## Technical Preview の意味

Technical Preview は、Fluno の方向性と公開 API surface を試せるようにするための段階です。

現時点では、次の性質を持ちます。

- API や仕様は変更される可能性があります。
- production use はまだ想定していません。
- binary は未署名の場合があります。
- compiler/runtime core はこのリポジトリには含まれていません。
- すべての `.fln` プログラムの互換性は保証していません。

## 配布物

GitHub Releases では、環境ごとに binary bundle を配布する想定です。

```text
fluno-linux-x86_64.tar.gz
fluno-macos-arm64.tar.gz
fluno-windows-x86_64.zip
```

bundle には、通常次のファイルが含まれます。

```text
bin/fluno または bin/fluno.exe
README
LICENSE / notices
SHA256SUMS.txt
PUBLIC_RELEASE_MANIFEST.json
docs
examples
```

## 今後の予定

公開範囲は、安定性と配布条件を確認しながら段階的に広げます。

1. Public OSS repository の整備。
2. limited public toolchain binary の配布。
3. artifact validation / execution の public contract 安定化。
4. frontend / LSP 公開範囲の再検討。
5. compiler/runtime core の公開形態の検討。
