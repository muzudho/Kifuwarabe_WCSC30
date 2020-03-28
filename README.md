# Rust Kifuwarabe WCSC30

Rust言語だぜ☆（＾～＾）　今回は　他のコンピューター将棋ソフトの流行り合わせていくぜ☆（＾～＾）

## Check

* [v] 256手制限ルールから、320手制限ルールに変更☆（＾～＾）

## Compile

Visual Studio Code `[Terminal] - [New Terminal]`.  

```Shell
cargo build --release
```

## Start on the Shogidokoro

Shogidokoro `[対局(G)] - [エンジン管理...] - [追加...]`  
`./target/release/rust-kifuwarabe-wcsc30.exe`

## Start on the terminal

Visual Studio Code `[Terminal] - [New Terminal]`.  

```Shell
cargo run --release
```

## エラーが出た局面

```
### 解消
position startpos moves 2h3h 5c5d 3h1h 8b5b 6i5h 5b6b 5g5f 7a7b 3g3f 4a5b 5h6h 3a3b 9g9f 5b4b 1h2h 5a4a 5f5e 5d5e 6h7h

### 金が後ろに下がる☆（＾～＾）
position startpos moves 2h7h 1c1d 6i6h 4c4d 4g4f 9c9d 6h6i 3c3d 4i3h 8b7b 7h6h 9a9c
```
