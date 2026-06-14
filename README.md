# Fluno

Fluno は、現実世界と融合するような次世代AIの実行基盤を目指して設計している言語です。

このリポジトリでは、Fluno の公開可能な部分として、構文解析 crate、artifact 仕様 crate、標準ライブラリの安定サブセット、サンプル、ドキュメントを公開しています。compiler/runtime の中核実装は、このリポジトリには含まれていません。

> Status: Public Technical Preview
> API、仕様、コマンド、artifact 形式は今後変更される可能性があります。

## クイックインストール

`.fln` ファイルを実際に `check` / `compile-artifact` / `run` するには、別途配布している Fluno public toolchain binary が必要です。

GitHub Releases から自分の環境に合う archive をダウンロードしてください。

| 環境 | archive |
|---|---|
| Linux x86_64 | `fluno-linux-x86_64.tar.gz` |
| macOS arm64 | `fluno-macos-arm64.tar.gz` |
| Windows x86_64 | `fluno-windows-x86_64.zip` |

Linux / macOS:

```bash
# 例: Linux x86_64
curl -L -o fluno-linux-x86_64.tar.gz \
  https://github.com/soichiro121/Fluno/releases/latest/download/fluno-linux-x86_64.tar.gz

tar -xzf fluno-linux-x86_64.tar.gz
./public-binary/bin/fluno version
```

macOS では、未署名 preview binary のため、初回実行時に Gatekeeper の警告が出る場合があります。

Windows PowerShell:

```powershell
Invoke-WebRequest `
  -Uri https://github.com/soichiro121/Fluno/releases/latest/download/fluno-windows-x86_64.zip `
  -OutFile fluno-windows-x86_64.zip

Expand-Archive .\fluno-windows-x86_64.zip -DestinationPath .\fluno
.\fluno\public-binary\bin\fluno.exe version
```

binary は Technical Preview です。正式な production use はまだ想定していません。配布物に含まれる `SHA256SUMS.txt` と `PUBLIC_RELEASE_MANIFEST.json` も確認してください。

## まず試す

このリポジトリに含まれる最小例を使うと、Fluno の基本的な流れを確認できます。

```fluno
fn main() -> Int {
    42
}
```

Linux / macOS:

```bash
FLUNO=./public-binary/bin/fluno

$FLUNO check examples/hello/hello.fln --json
$FLUNO compile-artifact examples/hello/hello.fln --out out/hello --json
$FLUNO artifact inspect out/hello/manifest.json --json
$FLUNO artifact validate out/hello/manifest.json --json
$FLUNO artifact run out/hello/manifest.json --json
```

Windows PowerShell:

```powershell
$FLUNO = ".\fluno\public-binary\bin\fluno.exe"

& $FLUNO check examples\hello\hello.fln --json
& $FLUNO compile-artifact examples\hello\hello.fln --out out\hello --json
& $FLUNO artifact inspect out\hello\manifest.json --json
& $FLUNO artifact validate out\hello\manifest.json --json
& $FLUNO artifact run out\hello\manifest.json --json
```

現在の public toolchain で公開している主なコマンドは次の通りです。

```text
fluno version
fluno check
fluno compile-artifact
fluno artifact inspect
fluno artifact validate
fluno artifact run
```

REPL、raw compile、Core IR / bytecode / MLIR dump、optimizer trace、内部 benchmark command は含めていません。

## このリポジトリでできること

Fluno compiler を使わずに、この公開 workspace だけで次のことを確認できます。

```bash
cargo fmt --check
cargo check --workspace
cargo test --workspace
```

主な用途は次の通りです。

- Fluno の構文表面を読む。
- `.fln` ファイルを構文レベルで解析する。
- artifact manifest / ABI / schema の公開データ構造を確認する。
- 標準ライブラリの公開シグネチャを確認する。
- サンプルコードや manifest 例を参照する。

## 含まれるもの

```text
crates/fluno-syntax
  Lexer / parser / AST / source map / syntax diagnostics

crates/fluno-artifact-spec
  Artifact manifest / ABI / schema / validation profile / hash utility

stdlib/stable
  core / math / probabilistic API surface の公開サブセット

docs
  構文概要、artifact 仕様、公開範囲、配布方針

examples
  最小例、代表的構文例、artifact manifest 例
```

## 含まれないもの

このリポジトリは、Fluno の完全な compiler/runtime 実装ではありません。次の実装は含まれていません。

- typechecker / semantic checker
- compiler pipeline / Core IR lowering
- Bytecode VM / runtime 実装
- MLIR backend / optimizer / target-specific lowering
- artifact builder / runner の内部実装
- tensor / accelerator / neuromorphic backend の詳細
- 非公開の benchmark / fixture / diagnostic command
- 開発用 CLI

## Rust crate として使う

このリポジトリ内の crate は、構文解析や artifact metadata tooling の基盤として使えます。

```toml
[dependencies]
fluno-syntax = { git = "https://github.com/soichiro121/Fluno", package = "fluno-syntax" }
fluno-artifact-spec = { git = "https://github.com/soichiro121/Fluno", package = "fluno-artifact-spec" }
```

`fluno-syntax` は構文解析までを扱います。typecheck、semantic validation、lowering、execution は行いません。

## ドキュメント

まず読むもの:

- [Language Reference](docs/reference/language.md)
- [Domain Syntax Reference](docs/reference/domain-syntax.md)
- [Toolchain Reference](docs/reference/toolchain.md)
- [Artifact Reference](docs/reference/artifact.md)
- [Standard Library Reference](docs/reference/stdlib.md)
- [Current Limitations](docs/reference/limitations.md)

補足ドキュメント:

- [構文概要](docs/syntax-overview.md)
- [Artifact 仕様](docs/artifact-spec.md)
- [公開範囲](docs/public-boundary.md)
- [配布方針](docs/release-strategy.md)

## ライセンス

このリポジトリで公開している OSS 層は `Apache-2.0 OR MIT` の dual license です。

- `LICENSE-APACHE`
- `LICENSE-MIT`

compiler/runtime core や binary toolchain には、このリポジトリとは異なる配布条件が適用される場合があります。
