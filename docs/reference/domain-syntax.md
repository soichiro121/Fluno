# Domain Syntax Reference

この文書は、Fluno Public Technical Preview で外部に見せるべき「Flunoらしい」ドメイン系構文をまとめたものです。

Fluno は一般的な式・関数・構造体だけを持つ言語ではなく、AI、確率的計算、リアクティブ処理、非同期実行、artifact 配布を同じ言語表面で扱うことを目指しています。この文書では、そのうち公開リポジトリから確認できる surface を説明します。

注意: ここで説明するものには、すでに安定サブセットに入っているものと、構文・型名として公開されているが runtime support が toolchain 側に依存するものがあります。`fluno-syntax` で parse できることと、すべての機能が安定して実行可能であることは同じではありません。

## 安定度の目安

| 領域 | 公開状態 | 説明 |
|---|---|---|
| 確率分布型 | 公開 stable subset | `Gaussian`, `Uniform`, `Bernoulli`, `Beta`, `VonMises` などの型名と一部 intrinsic signature を公開しています。 |
| 観測・sample API | 公開 stable subset | `stdlib/stable/prob.fln` に公開シグネチャがあります。 |
| `Signal<T>` / `Event<T>` | syntax surface | 型表面は公開されています。runtime semantics は toolchain 側に依存します。 |
| `stream { ... }` | syntax surface | stream block 構文は公開されています。安定実行仕様は今後整理されます。 |
| `Future<T>` / `async` / `spawn` / `.await` | syntax surface | 非同期・future系の構文表面は公開されています。 |
| `with name = expr { ... }` | syntax surface | ドメイン文脈・resource文脈を導入するための構文表面です。 |
| `EstimatorKind` / `SurrogateRule` | reserved domain types | AD・surrogate gradient 系のための型名として公開 syntax に含まれます。 |
| tensor ABI | artifact surface | source構文ではなく、artifact ABI上で `StaticTensor` / `DynamicTensorDescriptor` などを公開しています。 |

## 確率プログラミング surface

Fluno の確率系 surface は、分布を値として扱い、観測や sample を明示的な関数として記述する形を取ります。

公開サブセットでは、次の分布型が構文・型表面に含まれます。

```fluno
Gaussian
Uniform
Bernoulli
Beta
VonMises
```

`stdlib/stable/prob.fln` では、次の intrinsic signature を公開しています。

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

最小例:

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

観測の例:

```fluno
extern "intrinsic" {
    fn observe_normal(value: Float, mean: Float, sigma: Float) -> Float;
}

fn score_observation(x: Float) -> Float {
    observe_normal(x, 0.0, 1.0)
}
```

この公開surfaceで重要なのは、確率分布を単なるライブラリ関数ではなく、Fluno の型表面に現れるドメイン値として扱う点です。一方で、推論器、サンプリング戦略、log probability の内部表現、gradient との接続は compiler/runtime 側の責務であり、この公開リポジトリには含まれていません。

## リアクティブ surface: `Signal<T>` と `Event<T>`

Fluno の syntax surface には、時間変化する値や離散イベントを表す型が含まれます。

```fluno
Signal<Float>
Event<Bool>
```

構文例:

```fluno
fn keep_signal(x: Signal<Float>) -> Signal<Float> {
    x
}

fn keep_event(e: Event<Bool>) -> Event<Bool> {
    e
}
```

`Signal<T>` は継続的・時間的に変化する値、`Event<T>` は離散的に発生する値を表すための型表面です。Public Technical Preview では、型名と構文表面を公開していますが、reactive runtime、scheduler、event propagation、temporal consistency の実装は公開していません。

## stream block

`stream` は、stream を構文上の block として扱うためのキーワードです。

```fluno
fn stream_surface() {
    stream {
        1;
        2;
        3;
    }
}
```

`fluno-syntax` は `stream { ... }` を専用の AST node として扱います。これは、通常の関数呼び出しだけではなく、言語側に stream 構造を持たせるための surface です。

現時点では、stream の生成、消費、cancel、backpressure、event bridge などの runtime semantics は Public Technical Preview の安定仕様としては固定していません。公開文書では、`stream` を「syntax surface」として扱い、runtime詳細を過度に約束しないでください。

## 非同期 surface: `async`, `Future<T>`, `spawn`, `.await`

Fluno の syntax surface には、非同期処理を表す要素が含まれています。

```fluno
Future<Int>
async fn
spawn expr
expr.await
```

構文例:

```fluno
async fn answer() -> Int {
    42
}

async fn run() -> Int {
    let f: Future<Int> = spawn answer();
    f.await
}
```

`spawn` は式を future として起動するための構文表面です。`.await` は future の結果を待つ postfix 構文です。

公開時の説明では、これを「Fluno が将来的に concurrent / async workload を言語表面で扱うための構文」として位置づけるのが安全です。scheduler、task lifecycle、cancellation、sendability、ownership boundary の詳細は公開 compiler/runtime 側の実装に依存します。

## `with` block

`with` は、局所的な文脈を導入して block を評価するための構文です。

```fluno
fn scoped() -> Int {
    with scale = 2 {
        scale * 21
    }
}
```

この構文は、将来的に estimator、runtime policy、resource、effect context などを block に結び付けるための表面構文として使えます。

Public Technical Preview では、`with name = expr { ... }` という構文表面を公開しています。ただし、どの文脈値がどの runtime 効果を持つかは、公開 stable stdlib ではまだ広く固定していません。

## AD / surrogate gradient surface

Fluno の syntax surface には、AD や surrogate gradient 系の拡張に向けた型名が含まれています。

```fluno
EstimatorKind
SurrogateRule
```

これらは、確率推論、surrogate gradient、neuromorphic / spiking model などのドメインで使う余地がある型表面です。

ただし、Public Technical Preview の公開 OSS 層では、AD 実行器、reverse-mode transform、surrogate rule の runtime 実装、neuromorphic backend への lowering は含めていません。ドキュメント上は、「Fluno が想定しているドメイン拡張のための予約された public surface」として説明するのが適切です。

## artifact ABI に現れるドメイン型

Fluno の domain surface は source syntax だけではなく、artifact ABI にも現れます。

`fluno-artifact-spec` では、次のような ABI 型を公開しています。

```text
StaticTensor
DynamicTensorDescriptor
DistributionHandle
EventHandle
SignalHandle
StreamHandle
FunctionHandle
RuntimeHandle(...)
Opaque(...)
```

これは、Fluno artifact が単なる実行ファイルではなく、入力・出力・runtime接続・tensor descriptor・event/stream handle などを明示した配布単位であることを示します。

source-level の `Signal<T>` / `Event<T>` / probabilistic distribution と、artifact-level の `SignalHandle` / `EventHandle` / `DistributionHandle` は同一ではありません。前者は言語表面の型であり、後者はartifact境界に現れるABI表現です。