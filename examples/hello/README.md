# Hello

`hello.fln` は、Fluno の最小サンプルです。

```fluno
fn main() -> Int {
    42
}
```

public toolchain binary を使うと、次の流れを試せます。

```bash
fluno check examples/hello/hello.fln --json
fluno compile-artifact examples/hello/hello.fln --out out/hello --json
fluno artifact validate out/hello/manifest.json --json
fluno artifact run out/hello/manifest.json --json
```

このリポジトリ単体では、構文やサンプルの内容を確認できます。コンパイルと実行には GitHub Releases で配布する Fluno public toolchain が必要です。
