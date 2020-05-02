use crate::cosmic::playing::{Game, PosNums};
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::features::{HandAddress, PIECE_WHITE_SPACE};
use crate::cosmic::smart::square::*;
use crate::law::generate_move::Piece;
use crate::law::generate_move::Way;
use crate::spaceship::equipment::Beam;

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
    fn to_string2(piece: Option<Piece>) -> String {
        if let Some(piece_val) = piece {
            format!("{}", piece_val.meaning)
        } else {
            PIECE_WHITE_SPACE.to_string()
        }
    }
    /// 表示
    pub fn to_string(game: &Game, pos_nums: PosNums) -> String {
        let board = game.get_board(pos_nums);
        let ply = game.history.ply;
        let phase = game.history.get_friend();
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
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 1))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 2))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 3))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 4))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 5))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 6))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 7))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 8))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(9, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(8, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(7, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(6, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(5, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(4, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(3, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(2, 9))),
            GameRoom::to_string2(board.piece_at(&AbsoluteAddress::new(1, 9))),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            board.count_hand(HandAddress::Rook1),
            board.count_hand(HandAddress::Bishop1),
            board.count_hand(HandAddress::Gold1),
            board.count_hand(HandAddress::Silver1),
            board.count_hand(HandAddress::Knight1),
            board.count_hand(HandAddress::Lance1),
            board.count_hand(HandAddress::Pawn1),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            board.count_hand(HandAddress::Rook2),
            board.count_hand(HandAddress::Bishop2),
            board.count_hand(HandAddress::Gold2),
            board.count_hand(HandAddress::Silver2),
            board.count_hand(HandAddress::Knight2),
            board.count_hand(HandAddress::Lance2),
            board.count_hand(HandAddress::Pawn2),
            ply,
            phase,
            same_pos_count
        )
    }
}

/* TODO
/// レストルームはこちらだぜ☆（＾～＾）！
pub struct RestRoom {}
impl RestRoom {
    fn to_string2(number: isize) -> String {
        format!("{}", number)
    }
    /// 表示
    pub fn to_string(game: &Game, phase: Phase) -> String {
        let board = game.get_board(PosNums::Current);
        let ply = game.history.ply;

        // 局面表示
        format!(
            "\
[{95} ply. {96} phase.]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0:>4}|{1:>4}|{2:>4}|{3:>4}|{4:>4}|{5:>4}|{6:>4}|{7:>4}|{8:>4}| a1   ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9:>4}|{10:>4}|{11:>4}|{12:>4}|{13:>4}|{14:>4}|{15:>4}|{16:>4}|{17:>4}| b2   r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18:>4}|{19:>4}|{20:>4}|{21:>4}|{22:>4}|{23:>4}|{24:>4}|{25:>4}|{26:>4}| c3   b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27:>4}|{28:>4}|{29:>4}|{30:>4}|{31:>4}|{32:>4}|{33:>4}|{34:>4}|{35:>4}| d4   g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36:>4}|{37:>4}|{38:>4}|{39:>4}|{40:>4}|{41:>4}|{42:>4}|{43:>4}|{44:>4}| e5   s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45:>4}|{46:>4}|{47:>4}|{48:>4}|{49:>4}|{50:>4}|{51:>4}|{52:>4}|{53:>4}| f6   n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54:>4}|{55:>4}|{56:>4}|{57:>4}|{58:>4}|{59:>4}|{60:>4}|{61:>4}|{62:>4}| g7   l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63:>4}|{64:>4}|{65:>4}|{66:>4}|{67:>4}|{68:>4}|{69:>4}|{70:>4}|{71:>4}| h8   p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72:>4}|{73:>4}|{74:>4}|{75:>4}|{76:>4}|{77:>4}|{78:>4}|{79:>4}|{80:>4}| i9
        +----+----+----+----+----+----+----+----+----+\
        ",
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 1))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 2))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 3))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 4))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 5))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 6))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 7))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 8))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(9, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(8, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(7, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(6, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(5, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(4, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(3, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(2, 9))),
            RestRoom::to_string2(board.get_control(phase, &AbsoluteAddress::new(1, 9))),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            board.count_hand(HandAddress::Rook1),
            board.count_hand(HandAddress::Bishop1),
            board.count_hand(HandAddress::Gold1),
            board.count_hand(HandAddress::Silver1),
            board.count_hand(HandAddress::Knight1),
            board.count_hand(HandAddress::Lance1),
            board.count_hand(HandAddress::Pawn1),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            board.count_hand(HandAddress::Rook2),
            board.count_hand(HandAddress::Bishop2),
            board.count_hand(HandAddress::Gold2),
            board.count_hand(HandAddress::Silver2),
            board.count_hand(HandAddress::Knight2),
            board.count_hand(HandAddress::Lance2),
            board.count_hand(HandAddress::Pawn2),
            ply,
            phase,
        )
    }
}
*/

/// 台所はこちらだぜ☆（＾～＾）！指し手の一覧が見れるぜ☆（＾～＾）！
pub struct Kitchen {}
impl Kitchen {
    /// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
    pub fn print_ways(ways: &Vec<Way>) {
        Beam::shoot(&format!("Moves count={}", ways.len()));
        // 辞書順ソート
        let mut move_names = Vec::new();
        for way in ways {
            let ss_str = format!(
                "{}{}",
                format!("{}", way.movement),
                if let Some(psuedo_captured) = way.captured {
                    format!(" ({})", psuedo_captured.meaning)
                } else {
                    "".to_string()
                }
            );
            move_names.push(ss_str);
        }
        // move_names.sort();
        move_names.sort_by(|y_str, x_str| {
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
        move_names.reverse();

        for (i, move_name) in move_names.into_iter().enumerate() {
            Beam::shoot(&format!("[{}] {}", i, move_name));
        }
    }
}
