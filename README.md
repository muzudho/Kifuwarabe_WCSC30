# Rust Kifuwarabe WCSC30

Rust言語だぜ☆（＾～＾）　今回は　他のコンピューター将棋ソフトの流行り合わせていくぜ☆（＾～＾）

## 直近のTODO

* [ ] 駒に背番号付けろだぜ☆（＾～＾）

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

### 2段目の歩が怪しい☆（＾～＾）

position startpos moves 5g5f 6a7b 4i3h 5a4b 2g2f 4b3b 5i4i 4a5b 9g9f 7c7d 5f5e 8b9b 7i7h 7a8b 8i9g 3c3d 3g3f 7b7c 2h2g 3b3c 7g7f 8b7a 8h7i 1c1d 1i1h 7a8b 9g8e 7c8d 8e7c 3c4b 7h8i 3a3b 3h4h 4b3a 7f7e 7d7e 6i6h 2a3c 1g1f 2c2d 4i3h 3b2c 6h6i 2d2e 8g8f 7e7f 6g6f 3a4a 4h5g 1a1b 9i9g 1b1c 2i3g 2b1a 8f8e 8d7d 5g5f 3d3e 4g4f 8a7c 5e5d N*4b 5d5c 5b5a 8i8h 7d7e 3h4h 4a3b 5c5b+ 5a5b 3i3h P*5g 4h4g 9c9d 1h1g 3b4a 7i6h 7f7g 6f6e 2c3d 8h7g 3d2c 8e8d 4c4d P*5h 2c3b P*7b P*7a 3f3e 5b5c 6h5i 5g5h 2f2e 3b2a 7g6h P*2d 5f5e 7a7b P*7f 7e6f 2e2d 2a1b 5i4h 6f7f 6i5h P*2f 2g2f P*3d 2f2i 8b9c 3e3d 4b3d 1f1e 7f8f 4f4e P*2e 4h7e 5c5d 5e5f 8f8g P*7i 4a3b 9f9e 7c8e 9g9f 1d1e 5f6f 6c6d 8d8c+ 8g9h 6f7f 3b4b P*5b

### なんでここで動かなくなるんだぜ☆（＾～＾）？ あっ、３２０手制限かだぜ☆（＾～＾）！

position startpos moves 5g5f 6a7b 4i3h 5a4b 2g2f 4b3b 5i4i 4a5b 9g9f 7c7d 5f5e 8b9b 7i7h 7a8b 8i9g 3c3d 3g3f 7b7c 2h2g 3b3c 7g7f 8b7a 8h7i 1c1d 1i1h 7a8b 9g8e 7c8d 8e7c 3c4b 7h8i 3a3b 3h4h 4b3a 7f7e 7d7e 6i6h 2a3c 1g1f 2c2d 4i3h 3b2c 6h6i 2d2e 8g8f 7e7f 6g6f 3a4a 4h5g 1a1b 9i9g 1b1c 2i3g 2b1a 8f8e 8d7d 5g5f 3d3e 4g4f 8a7c 5e5d N*4b 5d5c 5b5a 8i8h 7d7e 3h4h 4a3b 5c5b+ 5a5b 3i3h P*5g 4h4g 9c9d 1h1g 3b4a 7i6h 7f7g 6f6e 2c3d 8h7g 3d2c 8e8d 4c4d P*5h 2c3b P*7b P*7a 3f3e 5b5c 6h5i 5g5h 2f2e 3b2a 7g6h P*2d 5f5e 7a7b P*7f 7e6f 2e2d 2a1b 5i4h 6f7f 6i5h P*2f 2g2f P*3d 2f2i 8b9c 3e3d 4b3d 1f1e 7f8f 4f4e P*2e 4h7e 5c5d 5e5f 8f8g P*7i 4a3b 9f9e 7c8e 9g9f 1d1e 5f6f 6c6d 8d8c+ 8g9h 6f7f 3b4b P*5b 7b7c 7i7h 9b5b 4g5g P*8g 7f8e P*5c N*1h 5d6e 7e8f P*3f P*6i 9a9b P*8b 1a2b 6h5i 1b2c 2i3i 4b3b 8c7b 2c2d 8f6h 3b4a 5h6g 5b5a P*1b 7c7d 3i1i 7d7e P*5h 8g8h+ 3h4i 9h8i 8b8a+ P*8g 7b7c 4a4b 8e7d 8h7h 6h8f 8i9i 7c6c 8g8h+ 6c6b 5a4a 6b7b 2b3a 7b7c 8h8g 8f7g 9i9h 7g4d P*8c 7d6c 7e7f P*3e 7h6i 5i4h 6e7e 8a9a 3f3g 1i2i 4a5a 9a9b 9c8d 6g5f 6d6e 5g6g N*1a 4h4g 8g8f L*3h 3a2b 9b8b 3d2f 8b7a 3g3h+ 4i3h 4b3a 3h4i 3a2a 4g3f 6e6f 6g7h 6i7i 7h6h 8d9c 3e3d 1c1d 5f5g L*4h 4i4h 8c8d 5g4f 2b1c L*8h 6f6g+ 6h7i 1a2c 7c7b 9h9i 9e9d 8f8e 3d3c+ P*9g N*2g 7f7g 3c2b 1c2b P*3c P*3e 2g3e 9g9h+ 9d9c 2d1c 5h5g 2c3e S*7c 7e6e P*2d N*4g P*8g 2a1b 7b8a 4g5i+ 8a8b 2f3h+ 2i1i 2b3a 4h5i P*6b 7c6b+ 5a4a 4d3e 9h9g 3f4g 3h2h 1i4i 9i9h N*2f 2e2f 1h2f N*7f P*1h 9g8h 7i6i 8e7e 4e4d L*7c N*8c 1b1a 7a8a 3a2b 9f9d 1a1b 2d2c+ 1b2c 8b9b P*2d P*6d 6g7h 6i5h 2h1h 4d4c P*4d 1g1e 7e8f 9b9a 1d1e 9c9b+ 7c7e P*2h 8f8e 3c3b+ 2c3b 6c7c 2b3c 9b8b L*9f 5g5f 4a3a 9a9b 1c2b 4i3i 8h8i 5h6g 7f8h+

### 王手が打てない☆（＾～＾）なんでだぜ☆（＾～＾）？

position startpos moves 7g7f 3c3d 8h7g 7a7b 2h9h 6a6b 7i6h 5a4b 7g8f 3d3e 3g3f 2b5e 8f7g 5e1i+ 7g1a+ L*5h 4i5h 4c4d 1a2a 6b6a L*4c 4b5b N*7d 3a3b 3i2h 9c9d 4c4b+ 4a4b 7d6b+ 5b4c 2a3b 4c3d S*4c 3d2e 4c3d 2e1d 3d2c 1d1e L*1f 1e2d 2c1d+ 2d2e 1d1e 2e3d 3b3c 3d4e 3c3d 4e3d 3f3e 3d2c 1e1d 2c1b 1d1c 1b2a 1c2b 2a2b 1f1c+ 2b3c 1c2c 3c4c 2c3c 4c5d 6b7b B*9e S*4c 5d6e 6g6f 6e7d 7b7c 9e7c 7f7e 7d8e 8i7g 8e9e 9g9f 9e8d 4c5d S*5a 5g5f 6a6b 7e7d N*5e 7d7c+ L*2b B*7e 8d7c 7g8e 7c7d 8e7c+ 7d7c 7e6d 7c7d P*7e 7d6d 5d5c 6d7c 5c6b 7c6d

### 王手を読むとフリーズする☆（＾～＾）？

position startpos moves 9i9h 8c8d 2h6h 1c1d 2g2f 1d1e 6g6f 5a6b 8g8f 8d8e 6h3h 8e8f 6i5h 8f8g+ 7i7h 8g8h 3h4h 8h7h

### 後手が歩を 0i に打ったぜ☆（＾～＾）？

position startpos moves 7g7f 3c3d 8g8f 4a5b 9i9h 6c6d 2g2f 5b4b 8h7g 2b4d 3g3f 3a2b 1g1f 6a7b 2h2g 4d3e 7g3c 3e2f 5i5h 2f1g+ 4i3h

### 後手が桂馬を 0c に打ったぜ☆（＾～＾）？

position startpos moves 7g7f 3c3d 2h7h 8c8d 9i9h 6a6b 7h1h 4a3b 8g8f 8b8c 8h7g 8d8e 7g8h 5a5b 8h6f 7c7d 6f8h 7a7b 9g9f 1c1d 1h5h 8c8d 8h6f 7b8c 5h2h 2b3c 7i7h 3a4b 2h4h 4b5a 6i7i 3c4d 1g1f 9a9b 5i6h 2a1c 1i1h 6b6a 4i5i 9c9d 5i6i 1d1e 3g3f 4d2b 3i2h 2b4d 2i1g 1e1f 5g5f 1f1g+ 4h5h N*4f 4g4f 6a7b N*6d 5b6b 6d5b+ 6b5b 1h1g N*8g 1g1d 1a1b 1d1c 3b2b N*6d 6c6d 9f9e N*9c 2h1g 2b1c 5h2h L*1a 6h7g 6d6e 4f4e 7b7a 8f8e 4d1g 7g8f 8d8e 8f9f 9d9e 9f9g 9e9f 9g9f P*9e 9f9g S*8f 9g8h 8f7g 8h9g 7g8f 9g8h 8f7g 8h9g 7g8f 9g8h 8f7g+ 6f7g 8g7i+ 7g8f G*7g 8h9i 7g8h 9i8h 7i8i 7h8i P*8g 8h7h N*6f 7h8g 8e8f 8g8f P*8e 8f9g B*6d 9g8h 6d9g+ 9h9g 9e9f R*1h 9f9g+ 8h7g 9g8g 7g6h 8g7g 6h5g 7g6g 5g4f 6f7h+ B*5h 4c4d S*9h 4d4e 4f5e L*5d 5e4e P*4d 4e4f 4d4e 4f4e P*4d 4e3d 1c2d 3d3c 2d3d 3c2c 3d3c 2c2b 3c3b 2b3b 7d7e G*3e 5b6b 8i8h 1g2f+ N*7d 6b6c G*4h 2f2g 6i5i P*3g P*2e 1b1g+ P*6d 6c7d 7f7e 7d7c 7e7d 7c7b 7d7c+ 7b6a 5h4i

```
