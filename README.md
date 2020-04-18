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

### なんか突然指さなくなったぜ☆（＾～＾）
position startpos moves 2h4h 5a6b 2g2f 4a4b 9i9h 7a7b 4g4f 6c6d 3g3f 6b7a 4h3h 1a1b 1i1h 6a5b 7g7f 3a3b 5i6h 7a6b 6i7h 6d6e 3h3g 6e6f 6g6f 5b5a 3g3h 8c8d 5g5f 2b3a 9g9f 8b8c 4i5h 4c4d 5h5i 6b5b 5f5e 3a2b 6h5g 5b6b 5g5h 4b4c 7f7e 7b6c 3f3e 5a4b 8h9g 6b5a 5i6i 5a5b 7h6g 6c7b 6i7h 2c2d 6g5f 3b4a 7h6g 5b6c 5h4h 4c5d 4h5g 5d6d 3e3d 7c7d 3h6h 6d5d 5e5d 3c3d 2i3g 2d2e G*7a 7b7c P*3e 4b3c 7a6a

### なんか突然指さなくなったぜ☆（＾～＾）
position startpos moves 7g7f 6a5b 2h3h 5a6a 3h6h 8b9b 6g6f 9c9d 3g3f 4a3b 2g2f 8a9c 6h4h 9b6b 9i9h 9d9e 7i7h 9c8e 4g4f 6b7b 6i6h 7c7d 3f3e 7b9b 8h9i 9b9d 1g1f 6a5a 5i6i 5b6b 7h6g 1c1d 6i5h 5a6a 2f2e 9d8d 5h6i 8d9d 6g7h 6b7b 1f1e 3b4b 2e2d 7a8b 4h4g 7b7a 3i3h 4c4d 4g2g 4b3b 6i5h 6a5b 4f4e 1a1b 2g2h 9d9c 3e3d 3a4b 6f6e 5b4c 2d2c 2b1a 1i1g 3c3d 1e1d 4c5b 5h4g 8c8d 7h6i 4b4c 4i5i 9a9b 4g5h P*1h 9i8h 3b3a 2h2e 3a3b 8h5e 6c6d P*3g 4c5d 5e6f 5b5a 2e2d 3b3a 5h6g 7a6b 2c2b+ 3a2b 3h2g P*2c 2d2e 6b7c 5i4i 7c6c 4i5i 8b8c 5g5f 3d3e 6h5h 5a6a 2g2f 2b3c 5i6h 9c9d 2e3e 3c2d 3e3b+ 8e9g+ 9h9g P*3c P*2g 9d9c 3b4b 6a7a 4b4a 7a8b 6f3i 1b1d P*1e 5d6e 5h5g 1a2b N*1f 2d3d 6i7h 2a1c 4a4b 8b9a 3i4h 2b1a 4b3b 9e9f 3b4a 9a8b 4h3i 1a2b 3g3f 8c7b 4a4b 2b1a 1e1d 8b7c L*1b 7b8c 4b5b 6c5d 4e4d P*4h 1d1c+ 8d8e 5g4g 3d4e N*3h 9f9g+ P*6c 8c8d 1c2c P*9a 5b3b 9c9d 4d4c 9b9c 8i7g L*2h P*9b 7c8c 8g8f 9g9h 7f7e 8d7c 6g5g 6e7f 3i2h 7d7e L*4b 9d8d 1f2d 5d4d 3b2a 8c7d 5g6f 4d3e 2f3g 7f6g 6f6g P*2f 1g1d 9c9d S*3a 9d9g+ 1d1c 8d8a 3g4h 8a8d P*7b 9h8h 7h6i 8d8c 8f8e 4e4d 6h5h 3e2e 2h1i 7c8b 6g6f 2e3e 4h5i 4d4c P*1e P*4d 2c3b 7d7c 5h4h 8b7a 6i5h 7a6b 6f5e 7c8b 2i1g 8h9i 5e6f 6b7a 8e8d 8c8d 2g2f 3c3d P*8g P*2g 4h5g 8b7b 5i4h 8d8a 5g6g 4c4b 1e1d 9a9b 3a4b 7b7c 6f5g 4d4e 3b3c 5c5d G*6h 7c7b 1i3g L*2h 4b4a 1a3c 6c6b 3e2e 5f5e 2e1e 5h4i P*8b 4g5f 3c4b 2a2c 9g9f 5g4g 7a6b 4a3b 6b5a 4h5i 5a6b 7g6e 4e4f 3g4f 7b6a 3b4a+ 6a7b 4a4b 7b6a 6h5h 9f9g 4f7i 9g9h 2c2b 8b8c B*9f 9h8i 7i6h 6a7b 4b3b 8a7a P*4h P*4f 5f4f 7a9a 4g5g 9i9h 6e7c+ 6b7c 2f2e N*9c P*7g 8i8h 9f5b 9a5a 5b9f+ 1e1d 5g5f 7b8b 4h4g 1d1e 6g5g P*4b 9f8f 7c8d 5h6g 5a6a 3b4b 8b9a 3f3e 5d5e 5f6f 6a8a P*1d P*4e
```
