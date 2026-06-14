# Language Reference

この文書は、Fluno Public Technical Preview で公開している言語表面を説明します。

ここで扱うのは、`.fln` ファイルを書くために必要な構文と、公開 crate `fluno-syntax` が扱う syntax surface です。typecheck、name resolution、semantic validation、lowering、runtime execution の詳細は、このリポジトリには含まれていません。

## ファイル構造

Fluno のソースファイルは、top-level item の並びとして扱われます。

代表的な item は次の通りです。

```fluno
fn main() -> Int {
    42
}

struct Point {
    x: Float,
    y: Float,
}

enum Status {
    Ready,
    Failed(String),
}
```

公開 syntax crate は、関数、構造体、列挙型、trait、impl、module、import、type alias、extern block などを AST として扱います。ただし、公開 toolchain で安定して使うべき範囲は、サンプルと `stdlib/stable` に現れている基本構文を中心に考えてください。

## コメント

行コメントは `//` で書きます。

```fluno
// answer を返す最小例
fn main() -> Int {
    42
}
```

## 基本リテラル

Fluno の基本的な literal には、整数、浮動小数、真偽値、文字列があります。

```fluno
fn literals() -> Int {
    let i = 42;
    let x = 3.14;
    let ok = true;
    let msg = "hello";
    i
}
```

主な基本型は次の通りです。

| 型 | 説明 |
|---|---|
| `Int` | 整数 |
| `Float` | 浮動小数 |
| `Bool` | 真偽値 |
| `String` | 文字列 |
| `Unit` | 値を返さない場合の型 |

## 関数

関数は `fn` で定義します。引数には型を付けます。戻り値の型は `->` で書けます。

```fluno
fn add(x: Int, y: Int) -> Int {
    x + y
}
```

構文上は戻り値の型注釈を省略できる場面もあります。ただし、public toolchain で `check` / `compile-artifact` / `artifact run` まで行う entry 関数では、戻り値型を明示してください。

```fluno
fn local_value() {
    42
}
```

block の最後の式が、その block の値になります。

## `let` binding

局所変数は `let` で束縛します。

```fluno
fn double(x: Int) -> Int {
    let y = x * 2;
    y
}
```

`let` は値に名前を付けるための基本構文です。型注釈やより複雑な pattern の扱いは、toolchain のバージョンに依存する可能性があります。

## block と式

Fluno では `{ ... }` で block を作ります。block の最後に置かれた式は、その block の結果として扱われます。

```fluno
fn compute() -> Int {
    let x = 10;
    let y = 20;
    x + y
}
```

セミコロンを付けた行は statement として扱われます。最後の結果にしたい値には、通常セミコロンを付けません。

## 算術・比較・論理演算

代表的な演算子は次の通りです。

| 種類 | 演算子 |
|---|---|
| 算術 | `+`, `-`, `*`, `/`, `%` |
| 比較 | `==`, `!=`, `<`, `<=`, `>`, `>=` |
| 論理 | `&&`, `||`, `!` |

```fluno
fn arithmetic(x: Int, y: Int) -> Int {
    let sum = x + y;
    sum * 2
}
```

## `if`

条件分岐は `if` / `else` で書きます。

```fluno
fn sign(x: Int) -> Int {
    if x < 0 {
        -1
    } else {
        1
    }
}
```

Fluno の `if` は式として扱える設計です。両branchの型が揃う必要があります。

## 構造体

`struct` で名前付きフィールドを持つ型を定義できます。

```fluno
struct Point {
    x: Float,
    y: Float,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
```

field access は、toolchain のsemantic checkerによって検査されます。`fluno-syntax` は構文としての field access を扱いますが、型の妥当性までは判断しません。

## 列挙型

`enum` でvariantを持つ型を定義できます。

```fluno
enum Status {
    Ready,
    Failed(String),
}
```

variantには値を持たないものと、payloadを持つものがあります。

## `match`

Fluno の構文表面には pattern matching が含まれます。ただし、Public Technical Preview では、複雑な pattern や網羅性検査の安定性は今後変更される可能性があります。

```fluno
fn status_code(status: Status) -> Int {
    match status {
        Status::Ready => 0,
        Status::Failed(_) => -1,
    }
}
```

`match` のsemantic validationは public syntax crate ではなく、toolchain 側で行われます。

## `extern "intrinsic"`

公開 standard library では、compiler/runtime が提供する機能を `extern "intrinsic"` として宣言しています。

```fluno
extern "intrinsic" {
    fn sqrt(x: Float) -> Float;
}
```

これは Fluno runtime によって解決される組み込み関数の公開シグネチャです。このリポジトリには、それらの実装は含まれていません。

## module / import

syntax crate は `mod`、`use`、`import` などの構文を扱います。ただし、Public Technical Preview の実用例では、単一ファイルまたはサンプル内の直接定義を中心にしています。

```fluno
mod math_helpers {
    fn id(x: Float) -> Float {
        x
    }
}
```

module resolution の詳細は、公開 compiler core の対象外です。


## Fluno特有のドメイン系構文

Fluno の特徴は、一般的な関数・構造体・列挙型だけでなく、確率分布、リアクティブ値、イベント、stream、future、artifact ABI などのドメインを言語・toolchainの surface に置いている点です。

代表的なドメイン系surfaceには、次のものがあります。

| 領域 | 主な構文・型 |
|---|---|
| 確率分布 | `Gaussian`, `Uniform`, `Bernoulli`, `Beta`, `VonMises` |
| 観測・sample | `observe_normal`, `observe_bernoulli`, `sample_float`, `sample_bool` |
| リアクティブ | `Signal<T>`, `Event<T>` |
| stream | `stream { ... }` |
| 非同期 | `async fn`, `Future<T>`, `spawn`, `.await` |
| ドメイン文脈 | `with name = expr { ... }` |
| AD / surrogate | `EstimatorKind`, `SurrogateRule` |

詳細は [Domain Syntax Reference](domain-syntax.md) を参照してください。

## 確率関連の公開surface

`stdlib/stable/prob.fln` では、確率分布と観測に関する公開シグネチャを提供しています。

```fluno
extern "intrinsic" {
    fn normal(mean: Float, sigma: Float) -> Gaussian;
    fn sample_float(distribution: Gaussian) -> Float;
}

fn draw() -> Float {
    let distribution = normal(0.0, 1.0);
    sample_float(distribution)
}
```

これは確率プログラミング機能の安定した公開surfaceの一部です。推論器、runtime、サンプリング実装、gradient処理などの内部実装は、このリポジトリには含まれていません。

## 構文parseとtoolchain checkの違い

`fluno-syntax` でparseできることと、`fluno check` で通ることは同じではありません。

- `fluno-syntax`: token化、parse、AST構築を行う。
- `fluno check`: name resolution、typecheck、semantic validationを含むtoolchain側の検査を行う。

そのため、構文として正しい `.fln` でも、型や意味上の理由で `fluno check` が失敗することがあります。
