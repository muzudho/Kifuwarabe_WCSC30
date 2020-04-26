use crate::cosmic::playing::{Game, PosNums};
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::Person;
use crate::cosmic::smart::features::{HandAddress, PieceMeaning, PIECE_WHITE_SPACE};
use crate::cosmic::smart::square::*;
use crate::cosmic::toy_box::PieceNum;
use crate::law::speed_of_light::SpeedOfLight;
use crate::spaceship::equipment::Beam;
use std::collections::HashSet;
use std::hash::BuildHasher;

/// 指令室はこちらだぜ☆（＾～＾）！
pub struct CommandRoom {}
impl CommandRoom {
    // 対話モードのタイトル画面
    pub fn print_title() {
        // 横幅は 半角79文字使えるぜ☆（＾～＾）
        // 80文字目を使うと、次の行が改行で空行になってしまう☆（＾～＾）
        Beam::shoot(
            &"\
+--------- --------- --------- --------- --------- --------- --------- -------+
| KifuWarabe Shogi 2020                                                       |
+---------+--------- --------- --------- --------- --------- --------- -------+
          | Created by Muzudho (Doujin Circle Grayscale)                      |
          +--------- --------- --------- --------- --------- --------- -------+
05
          [Enter]
07
08
09
10
11
12
13
14
15
16
17
18
19
20
21
22
23\
"
            .to_string(),
        );
    }
}

/// ゲームルームはこちらだぜ☆（＾～＾）！
pub struct GameRoom {}
impl GameRoom {
    fn to_string2(piece: Option<(PieceMeaning, PieceNum)>) -> String {
        if let Some(piece_val) = piece {
            format!("{}", piece_val.0)
        } else {
            PIECE_WHITE_SPACE.to_string()
        }
    }
    /// 表示
    pub fn to_string(game: &Game, pos_nums: &PosNums, speed_of_light: &SpeedOfLight) -> String {
        let board = game.get_board(pos_nums);
        let ply = game.history.ply;
        let phase = game.history.get_phase(Person::Friend);
        let same_pos_count = game.count_same_position();

        // 局面表示
        format!(
            "\
[{95} ply. {96} phase. {97} repeats.]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}| a1   ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}| b2   r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}| c3   b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}| d4   g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}| e5   s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}| f6   n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}| g7   l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}| h8   p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}| i9
        +----+----+----+----+----+----+----+----+----+\
",
            GameRoom::to_string2(board.piece_at(&Address::new(9, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 1).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 2).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 3).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 4).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 5).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 6).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 7).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 8).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(9, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(8, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(7, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(6, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(5, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(4, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(3, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(2, 9).abs())),
            GameRoom::to_string2(board.piece_at(&Address::new(1, 9).abs())),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            board.count_hand(HandAddress::Rook1, speed_of_light),
            board.count_hand(HandAddress::Bishop1, speed_of_light),
            board.count_hand(HandAddress::Gold1, speed_of_light),
            board.count_hand(HandAddress::Silver1, speed_of_light),
            board.count_hand(HandAddress::Knight1, speed_of_light),
            board.count_hand(HandAddress::Lance1, speed_of_light),
            board.count_hand(HandAddress::Pawn1, speed_of_light),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            board.count_hand(HandAddress::Rook2, speed_of_light),
            board.count_hand(HandAddress::Bishop2, speed_of_light),
            board.count_hand(HandAddress::Gold2, speed_of_light),
            board.count_hand(HandAddress::Silver2, speed_of_light),
            board.count_hand(HandAddress::Knight2, speed_of_light),
            board.count_hand(HandAddress::Lance2, speed_of_light),
            board.count_hand(HandAddress::Pawn2, speed_of_light),
            ply,
            phase,
            same_pos_count
        )
    }
}

/// 台所はこちらだぜ☆（＾～＾）！指し手の一覧が見れるぜ☆（＾～＾）！
pub struct Kitchen {}
impl Kitchen {
    /// 指し手
    pub fn print_move_hashset<S: BuildHasher>(move_hashset: &HashSet<u64, S>) {
        Beam::shoot(&format!("ss_hashset.len()={}", move_hashset.len()));
        // 辞書順ソート
        let mut vec_ss_str = Vec::new();
        for ss_hash in move_hashset {
            let ss = Movement::from_hash(*ss_hash);
            let ss_str = format!("{}", ss);
            vec_ss_str.push(ss_str);
        }
        //vec_ss_str.sort();
        vec_ss_str.sort_by(|y_str, x_str| {
            let y_arr: Vec<_> = y_str.chars().collect();
            let x_arr: Vec<_> = x_str.chars().collect();
            use std::cmp::min;
            let len = min(y_arr.len(), x_arr.len());

            use std::cmp::Ordering;
            for i in 0..len {
                match x_arr[i].cmp(&y_arr[i]) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                }
            }

            // Returns Ordering::Greater, Ordering::Less, Ordering::Equal.
            x_arr.len().cmp(&y_arr.len())
        });
        vec_ss_str.reverse();

        for (i, ss_str) in vec_ss_str.into_iter().enumerate() {
            Beam::shoot(&format!("[{}] {}", i, ss_str));
        }
    }
}
