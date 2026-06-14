# Standard Library Reference

この文書は、`stdlib/stable/` で公開している Fluno standard library の安定サブセットを説明します。

このリポジトリに含まれる standard library は、Public Technical Preview で外部に見せるための **公開シグネチャ** です。実装は compiler/runtime 側で解決されます。

## 構成

```text
stdlib/stable/
  core.fln
  math.fln
  prob.fln
```

## `core.fln`

`core.fln` は、基本的な組み込み関数と汎用データ型の公開surfaceです。

### 出力・停止

```fluno
extern "intrinsic" {
    fn print(value: String) -> Unit;
    fn println(value: String) -> Unit;
    fn panic(message: String) -> Unit;
}
```

| 関数 | 説明 |
|---|---|
| `print(value: String) -> Unit` | 文字列を出力します。改行の扱いはruntimeに依存します。 |
| `println(value: String) -> Unit` | 文字列を出力します。通常は改行付きの出力として扱われます。 |
| `panic(message: String) -> Unit` | 実行を失敗として扱います。 |

### `Option<T>`

値が存在する場合と存在しない場合を表します。

```fluno
enum Option<T> {
    Some(T),
    None,
}
```

### `Result<T, E>`

成功値または失敗値を表します。

```fluno
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## `math.fln`

`math.fln` は、数値計算でよく使う数学関数の公開シグネチャです。

```fluno
extern "intrinsic" {
    fn abs(x: Float) -> Float;
    fn sqrt(x: Float) -> Float;
    fn exp(x: Float) -> Float;
    fn log(x: Float) -> Float;
    fn sin(x: Float) -> Float;
    fn cos(x: Float) -> Float;
    fn tanh(x: Float) -> Float;
    fn stable_sigmoid(x: Float) -> Float;
    fn clamp(x: Float, lo: Float, hi: Float) -> Float;
    fn lerp(a: Float, b: Float, t: Float) -> Float;
}
```

| 関数 | 説明 |
|---|---|
| `abs` | 絶対値を返します。 |
| `sqrt` | 平方根を返します。 |
| `exp` | 指数関数を返します。 |
| `log` | 自然対数を返します。 |
| `sin` | 正弦を返します。 |
| `cos` | 余弦を返します。 |
| `tanh` | 双曲線正接を返します。 |
| `stable_sigmoid` | 数値的に安定なsigmoidを返します。 |
| `clamp` | 値を `[lo, hi]` の範囲に制限します。 |
| `lerp` | `a` と `b` の線形補間を返します。 |

例:

```fluno
extern "intrinsic" {
    fn sqrt(x: Float) -> Float;
}

fn hypotenuse_square_root(x: Float) -> Float {
    sqrt(x)
}
```

## `prob.fln`

`prob.fln` は、確率分布、観測、サンプリングに関する公開シグネチャです。

```fluno
extern "intrinsic" {
    fn normal(mean: Float, sigma: Float) -> Gaussian;
    fn uniform(low: Float, high: Float) -> Uniform;
    fn bernoulli(p: Float) -> Bernoulli;
    fn beta(alpha: Float, beta: Float) -> Beta;
    fn von_mises(mu: Float, kappa: Float) -> VonMises;
    fn observe_normal(value: Float, mean: Float, sigma: Float) -> Float;
    fn observe_bernoulli(value: Bool, p: Float) -> Float;
    fn sample_float(distribution: Gaussian) -> Float;
    fn sample_bool(distribution: Bernoulli) -> Bool;
}
```

| 関数 | 説明 |
|---|---|
| `normal` | Gaussian distribution を作ります。 |
| `uniform` | Uniform distribution を作ります。 |
| `bernoulli` | Bernoulli distribution を作ります。 |
| `beta` | Beta distribution を作ります。 |
| `von_mises` | Von Mises distribution を作ります。 |
| `observe_normal` | 正規分布に対する観測スコアを扱います。 |
| `observe_bernoulli` | Bernoulli分布に対する観測スコアを扱います。 |
| `sample_float` | 浮動小数値をsampleします。 |
| `sample_bool` | 真偽値をsampleします。 |

例:

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

## 注意事項

`stdlib/stable` は、Fluno の全standard libraryではありません。stream、reactive、tensor、AD、neuromorphic backend向けAPIなどは、この公開サブセットには含めていません。

Public Technical Preview では、標準ライブラリのシグネチャや分類が変更される可能性があります。
