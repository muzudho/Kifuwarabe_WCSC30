# Rust Kifuwarabe WCSC30

(2020-12-16 wed) チェビシェフ距離にちなんで、 kifuwaracheby という名前を与えたぜ☆（＾～＾） ここから再開発な☆（＾～＾）

|                         | ファイル                                                              |
| ----------------------- | --------------------------------------------------------------------- |
| ソース                  | `Kifuwarabe_WCSC30/Cargo.toml`                                        |
| 将棋エンジン ソース     | `Kifuwarabe_WCSC30/src/main.rs`                                       |
| GUI                     | なし                                                                  |
| 将棋エンジン ランタイム | `Kifuwarabe_WCSC30/target/release/grayscale_kifuwaracheby_engine.exe` |
| 設定ファイル1           | `Kifuwarabe_WCSC30/grayscale_kifuwaracheby_engine.exe.config.toml`    |
| 設定ファイル2           | `Kifuwarabe_WCSC30/profile/Engine.toml`                               |

* `Kifuwarabe_WCSC30` のトップ・ディレクトリーに `Logs` ディレクトリーを作成してください。
* `cargo build --release` コマンドを打鍵して `将棋エンジン ランタイム` を生成してください。
* 設定ファイル1 を、 将棋エンジン ランタイムと同じディレクトリーに 置いてください。
* 設定ファイル1 の `profile = "./profile"` ファイルパスを 設定ファイル2のディレクトリーに合わせてください。

## Build

```shell
rustup update
cargo build --release
```

## Run

```shell
cargo run --release
# cargo run
```

## Manual

Rust言語だぜ☆（＾～＾）　今回は　他のコンピューター将棋ソフトの流行り合わせていくぜ☆（＾～＾）

## エンジン設定の例

```plain
DepthNotToGiveUp      [    7 ]    MaxDepth        [    5 ]
MinThinkSec           [    5 ]    MaxThinkSec     [   17 ]
KomawariWeightPer1000 [ 1000 ]    ManyWaysPer1000 [ 1000 ]
PromotionWeightPer1000[ 1000 ]
```

## 直近のTODO

(^_^)

## Compile

Visual Studio Code `[Terminal] - [New Terminal]`.  

```Shell
cargo build --release
```

## Start on the Shogidokoro

Shogidokoro `[対局(G)] - [エンジン管理...] - [追加...]`  
`./target/release/rust-kifuwarabe-wcsc30.exe`

## Engine option example

```Plain
大会設定例。
DepthNotToGiveUp 5 - 最低でも5手は読む
MaxDepth 6 - 6手読んだら打切り(優先度高)
MinThinkSec 30 - 最低でも30秒は読む
MaxThinkSec 35 - 35秒読んだら打切り(優先度高)
KomawariWeightPer1000 1000 - 駒割評価値100.0%反映
ManyWaysPer1000 10 - 選択肢の多い（非一直線な）局面好む。変な手になるばかり。001.0%反映
PromotionWeightPer1000 1000 - 成りに加点100.0%反映
```

## Debug on the terminal

Visual Studio Code `[Terminal] - [New Terminal]`.  

```Shell
cargo run
```

## Engin options

* `BoardCoverageWeightPer1000` - 千分率。相手と比べて、盤面に利かせている数。
* `DepthNotToGiveUp` - 思考時間の上限を無視して考え続ける深さ。MaxThinkSec より強い。玉を取られたら負けと判定しているので、王手放置漏れを回避するために最低５手（ディスカバード・アタックを検知するのに５手）は探索する必要があるぜ☆（＾～＾）そして６手は読まないと王手のために角をすぐ切るが、５手を読み切れるほどの探索速度は無いぜ☆（＾～＾）でも結局一手詰め判定が無いと、王手を無視して末端局面まで読むから無駄だぜ☆（＾～＾）
* `KomawariWeightPer1000` - 千分率。駒割。
* `PromotionWeightPer1000` - 千分率。成らないよりは成った方がお得だぜ、ということを教えるだけだぜ☆（＾～＾）大きくすると、歩と交換で角が成り込むぜ☆（＾～＾）
* `MaxDepth` - この深さを読んだら決断。
* `MaxThinkSec` - 秒。１手にかける思考時間の上限。
* `MinThinkSec` - 秒。１手にかける思考時間の下限。MaxDepth より弱い。

## Done

* [x] 320手制限ルールに対応☆（＾～＾）

## Documents

[Design](./doc/design.md)  
[Test](./doc/test.md)  

## References

* [Is it possible to implement methods on type aliases?](https://stackoverflow.com/questions/35568871/is-it-possible-to-implement-methods-on-type-aliases)
