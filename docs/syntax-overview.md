# Fluno 構文概要

この文書では、公開 crate `fluno-syntax` が扱う Fluno の構文表面を説明します。

`fluno-syntax` は、`.fln` source text を token に分解し、module / item / expression / pattern などを AST として構築するための crate です。editor tooling、静的解析、サンプル検査、外部レビューのために、構文レベルの API を公開しています。

## 対象範囲

`fluno-syntax` が提供する範囲は次の通りです。

- lexing
- parsing
- AST node type
- syntax diagnostics
- parse error
- source map utility

この crate は「Fluno の構文を読む」ための公開境界です。

## 対象外の範囲

`fluno-syntax` は compiler frontend 全体ではありません。次の処理は行いません。

- name resolution
- typecheck
- semantic validation
- Core IR lowering
- optimization
- bytecode / native / MLIR code generation
- runtime execution

そのため、`fluno-syntax` で parse に成功した `.fln` ファイルでも、Fluno toolchain の `check` で semantic error になる場合があります。

## 最小例

```fluno
fn main() -> Int {
    42
}
```

この例は `examples/hello/hello.fln` にあります。

## 代表的な構文例

`examples/representative-corpus/` には、Fluno の構文表面を確認するための小さなサンプルを置いています。

- `arithmetic.fln`
- `struct_enum.fln`
- `probability_minimal.fln`
- `reactive_minimal.fln`

これらは benchmark ではなく、構文と公開 API surface を確認するためのサンプルです。

## Rust から利用する

```rust
use fluno_syntax::parse_module;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
fn main() -> Int {
    42
}
"#;

    let module = parse_module(source)?;
    println!("{module:#?}");
    Ok(())
}
```

実際の API は crate の version に合わせて確認してください。


## 関連リファレンス

より体系的な説明は [Language Reference](reference/language.md) を参照してください。


## ドメイン系構文

Fluno には、確率分布、`Signal<T>` / `Event<T>`、`stream { ... }`、`Future<T>`、`spawn`、`.await` など、AI・リアクティブ処理・非同期実行を意識した構文表面があります。

詳細は [Domain Syntax Reference](reference/domain-syntax.md) を参照してください。
