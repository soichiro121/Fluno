# fluno-syntax

`fluno-syntax` は、Fluno の lexer、parser、AST、source map、syntax diagnostics を提供する Rust crate です。

Fluno の `.fln` source を構文レベルで読み取り、外部 tooling やサンプル検査に使える公開 API を提供します。

## 使い方

```toml
[dependencies]
fluno-syntax = { git = "https://github.com/soichiro121/Fluno", package = "fluno-syntax" }
```

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

## 提供するもの

- tokenization
- parser
- AST node type
- parse error
- syntax diagnostics
- source map utility

## 提供しないもの

この crate は構文解析用です。次の処理は行いません。

- typecheck
- semantic validation
- lowering
- optimization
- compile
- execution
- runtime semantics validation

`.fln` ファイルが parse に成功しても、Fluno toolchain の `check` で semantic error になる場合があります。

## ライセンス

`Apache-2.0 OR MIT`
