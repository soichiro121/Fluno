# Toolchain Reference

この文書は、Fluno Public Technical Preview で配布している limited public toolchain binary の使い方を説明します。

このリポジトリには compiler/runtime 本体は含まれていません。`.fln` ファイルを実際に検査・artifact化・実行するには、GitHub Releases で配布している `fluno` binary を使います。

## インストール

Release ページから、自分の環境に合う archive をダウンロードしてください。

| 環境 | archive |
|---|---|
| Linux x86_64 | `fluno-linux-x86_64.tar.gz` |
| macOS arm64 | `fluno-macos-arm64.tar.gz` |
| Windows x86_64 | `fluno-windows-x86_64.zip` |

Linux / macOS:

```bash
tar -xzf fluno-linux-x86_64.tar.gz
./public-binary/bin/fluno version
```

Windows PowerShell:

```powershell
Expand-Archive .\fluno-windows-x86_64.zip -DestinationPath .\fluno
.\fluno\public-binary\bin\fluno.exe version
```

現在のbinaryは未署名のTechnical Previewです。本番利用はまだ推奨していません。配布物には、確認用の `SHA256SUMS.txt` と、build内容を示す `PUBLIC_RELEASE_MANIFEST.json` が含まれています。

## 公開コマンド一覧

Public Technical Preview で公開している主なコマンドは次の通りです。

```text
fluno version
fluno check
fluno compile-artifact
fluno artifact inspect
fluno artifact validate
fluno artifact run
```

公開していないもの:

```text
REPL
raw compile
Core IR dump
bytecode dump
MLIR dump
lowering trace
optimizer trace
non-public benchmark command
development-only CLI
```

## `fluno version`

toolchain のバージョン情報を表示します。

```bash
fluno version
```

配布bundleの内容やtarget情報は、bundle内の `PUBLIC_RELEASE_MANIFEST.json` も確認してください。

## `fluno check`

`.fln` ファイルを検査します。

```bash
fluno check examples/hello/hello.fln --json
```

`check` は、構文だけではなく、toolchain側のsemantic validationも含めて検査します。`fluno-syntax` でparseできるファイルでも、`check` では失敗する場合があります。

## `fluno compile-artifact`

`.fln` ファイルから Fluno artifact を生成します。

```bash
fluno compile-artifact examples/hello/hello.fln --out out/hello --json
```

生成先には、少なくともartifact metadataを表す `manifest.json` が含まれます。artifactの内部payloadや実行形式の詳細は、公開APIではありません。

## `fluno artifact inspect`

artifact manifest を読み、内容を表示します。

```bash
fluno artifact inspect out/hello/manifest.json --json
```

主に、artifactのidentity、target、ABI、schema、validation profileなどを確認するために使います。

## `fluno artifact validate`

artifact manifest と公開metadataを検証します。

```bash
fluno artifact validate out/hello/manifest.json --json
```

これは、artifactが公開metadata contractに沿っているかを確認するためのコマンドです。

## `fluno artifact run`

artifactを実行します。

```bash
fluno artifact run out/hello/manifest.json --json
```

Public Technical Preview では、実行可能なartifactやruntime機能の範囲は限定されています。失敗時は、JSON出力のdiagnosticを確認してください。

## 一連の流れ

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

## JSON出力

公開toolchainでは、CIや外部ツールから扱いやすいように `--json` を利用できます。JSON出力は、成功/失敗、diagnostic、artifact path、metadataなどを機械的に読むためのものです。

Public Technical Preview では、JSON schemaも今後変更される可能性があります。
