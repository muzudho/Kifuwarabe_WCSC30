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

### なんか強制終了したぜ☆（＾～＾）
position startpos moves 2h4h 5a6b 2g2f 4a4b 9i9h 7a7b 4g4f 6c6d 3g3f 6b7a 4h3h 1a1b 1i1h 6a5b 7g7f 3a3b 5i6h 7a6b 6i7h 6d6e 3h3g 6e6f 6g6f 5b5a 3g3h 8c8d 5g5f 2b3a 9g9f 8b8c 4i5h 4c4d 5h5i 6b5b 5f5e 3a2b 6h5g 5b6b 5g5h 4b4c 7f7e 7b6c 3f3e 5a4b 8h9g 6b5a 5i6i 5a5b 7h6g 6c7b 6i7h 2c2d 6g5f 3b4a 7h6g 5b6c 5h4h 4c5d 4h5g 5d6d 3e3d 7c7d 3h6h 6d5d 5e5d 3c3d 2i3g 2d2e G*7a 7b7c P*3e 4b3c 7a6a
```
