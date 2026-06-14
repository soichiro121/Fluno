# Representative Corpus

このディレクトリには、Fluno の代表的な構文を示す小さな `.fln` ファイルを置いています。

これらは performance benchmark ではありません。構文表面、API surface、サンプルとしての読みやすさを確認するための corpus です。

## ファイル

- `arithmetic.fln`: expression、let binding、function return の基本例。
- `struct_enum.fln`: struct / enum declaration と struct literal の例。
- `probability_minimal.fln`: probabilistic API surface の最小例。
- `reactive_minimal.fln`: `Signal<T>` / `Event<T>` の型表面と、実行可能な最小 entry の例。

## 使い方

構文解析 crate の検査対象として読むことも、public toolchain binary で `check` することもできます。各ファイルには `check` 用の小さな `main` を含めています。

```bash
fluno check examples/representative-corpus/arithmetic.fln --json
```

Technical Preview のため、サンプルや API は今後変更される可能性があります。
