use crate::entities::cosmic::playing::{Game, PosNums};
use crate::entities::cosmic::smart::features::{HandPiece, PIECE_WHITE_SPACE};
use crate::entities::spaceship::equipment::Beam;
use crate::movegen::PieceEx;

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
        fn to_string2(pc_ex: Option<PieceEx>) -> String {
                if let Some(pc_ex) = pc_ex {
                        format!("{}", pc_ex.piece)
                } else {
                        PIECE_WHITE_SPACE.to_string()
                }
        }
        /// 表示
        pub fn to_string(game: &Game, pos_nums: PosNums) -> String {
                let position = game.get_board(pos_nums);
                let ply = game.history.moves_num();
                let phase = game.history.get_phase();
                let same_pos_count = game.count_same_position();

                // 局面表示
                format!(
                        "\
[{95} ply. {96} phase. {97} repeats.]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}| a    ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}| b    r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}| c    b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}| d    g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}| e    s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}| f    n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}| g    l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}| h    p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}| i
        +----+----+----+----+----+----+----+----+----+\
",
                        GameRoom::to_string2(position.piece_at_board(91u8)),
                        GameRoom::to_string2(position.piece_at_board(81u8)),
                        GameRoom::to_string2(position.piece_at_board(71u8)),
                        GameRoom::to_string2(position.piece_at_board(61u8)),
                        GameRoom::to_string2(position.piece_at_board(51u8)),
                        GameRoom::to_string2(position.piece_at_board(41u8)),
                        GameRoom::to_string2(position.piece_at_board(31u8)),
                        GameRoom::to_string2(position.piece_at_board(21u8)),
                        GameRoom::to_string2(position.piece_at_board(11u8)),
                        GameRoom::to_string2(position.piece_at_board(92u8)),
                        GameRoom::to_string2(position.piece_at_board(82u8)),
                        GameRoom::to_string2(position.piece_at_board(72u8)),
                        GameRoom::to_string2(position.piece_at_board(62u8)),
                        GameRoom::to_string2(position.piece_at_board(52u8)),
                        GameRoom::to_string2(position.piece_at_board(42u8)),
                        GameRoom::to_string2(position.piece_at_board(32u8)),
                        GameRoom::to_string2(position.piece_at_board(22u8)),
                        GameRoom::to_string2(position.piece_at_board(12u8)),
                        GameRoom::to_string2(position.piece_at_board(93u8)),
                        GameRoom::to_string2(position.piece_at_board(83u8)),
                        GameRoom::to_string2(position.piece_at_board(73u8)),
                        GameRoom::to_string2(position.piece_at_board(63u8)),
                        GameRoom::to_string2(position.piece_at_board(53u8)),
                        GameRoom::to_string2(position.piece_at_board(43u8)),
                        GameRoom::to_string2(position.piece_at_board(33u8)),
                        GameRoom::to_string2(position.piece_at_board(23u8)),
                        GameRoom::to_string2(position.piece_at_board(13u8)),
                        GameRoom::to_string2(position.piece_at_board(94u8)),
                        GameRoom::to_string2(position.piece_at_board(84u8)),
                        GameRoom::to_string2(position.piece_at_board(74u8)),
                        GameRoom::to_string2(position.piece_at_board(64u8)),
                        GameRoom::to_string2(position.piece_at_board(54u8)),
                        GameRoom::to_string2(position.piece_at_board(44u8)),
                        GameRoom::to_string2(position.piece_at_board(34u8)),
                        GameRoom::to_string2(position.piece_at_board(24u8)),
                        GameRoom::to_string2(position.piece_at_board(14u8)),
                        GameRoom::to_string2(position.piece_at_board(95u8)),
                        GameRoom::to_string2(position.piece_at_board(85u8)),
                        GameRoom::to_string2(position.piece_at_board(75u8)),
                        GameRoom::to_string2(position.piece_at_board(65u8)),
                        GameRoom::to_string2(position.piece_at_board(55u8)),
                        GameRoom::to_string2(position.piece_at_board(45u8)),
                        GameRoom::to_string2(position.piece_at_board(35u8)),
                        GameRoom::to_string2(position.piece_at_board(25u8)),
                        GameRoom::to_string2(position.piece_at_board(15u8)),
                        GameRoom::to_string2(position.piece_at_board(96u8)),
                        GameRoom::to_string2(position.piece_at_board(86u8)),
                        GameRoom::to_string2(position.piece_at_board(76u8)),
                        GameRoom::to_string2(position.piece_at_board(66u8)),
                        GameRoom::to_string2(position.piece_at_board(56u8)),
                        GameRoom::to_string2(position.piece_at_board(46u8)),
                        GameRoom::to_string2(position.piece_at_board(36u8)),
                        GameRoom::to_string2(position.piece_at_board(26u8)),
                        GameRoom::to_string2(position.piece_at_board(16u8)),
                        GameRoom::to_string2(position.piece_at_board(97u8)),
                        GameRoom::to_string2(position.piece_at_board(87u8)),
                        GameRoom::to_string2(position.piece_at_board(77u8)),
                        GameRoom::to_string2(position.piece_at_board(67u8)),
                        GameRoom::to_string2(position.piece_at_board(57u8)),
                        GameRoom::to_string2(position.piece_at_board(47u8)),
                        GameRoom::to_string2(position.piece_at_board(37u8)),
                        GameRoom::to_string2(position.piece_at_board(27u8)),
                        GameRoom::to_string2(position.piece_at_board(17u8)),
                        GameRoom::to_string2(position.piece_at_board(98u8)),
                        GameRoom::to_string2(position.piece_at_board(88u8)),
                        GameRoom::to_string2(position.piece_at_board(78u8)),
                        GameRoom::to_string2(position.piece_at_board(68u8)),
                        GameRoom::to_string2(position.piece_at_board(58u8)),
                        GameRoom::to_string2(position.piece_at_board(48u8)),
                        GameRoom::to_string2(position.piece_at_board(38u8)),
                        GameRoom::to_string2(position.piece_at_board(28u8)),
                        GameRoom::to_string2(position.piece_at_board(18u8)),
                        GameRoom::to_string2(position.piece_at_board(99u8)),
                        GameRoom::to_string2(position.piece_at_board(89u8)),
                        GameRoom::to_string2(position.piece_at_board(79u8)),
                        GameRoom::to_string2(position.piece_at_board(69u8)),
                        GameRoom::to_string2(position.piece_at_board(59u8)),
                        GameRoom::to_string2(position.piece_at_board(49u8)),
                        GameRoom::to_string2(position.piece_at_board(39u8)),
                        GameRoom::to_string2(position.piece_at_board(29u8)),
                        GameRoom::to_string2(position.piece_at_board(19u8)),
                        //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
                        position.count_hand(HandPiece::Rook1),
                        position.count_hand(HandPiece::Bishop1),
                        position.count_hand(HandPiece::Gold1),
                        position.count_hand(HandPiece::Silver1),
                        position.count_hand(HandPiece::Knight1),
                        position.count_hand(HandPiece::Lance1),
                        position.count_hand(HandPiece::Pawn1),
                        //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
                        position.count_hand(HandPiece::Rook2),
                        position.count_hand(HandPiece::Bishop2),
                        position.count_hand(HandPiece::Gold2),
                        position.count_hand(HandPiece::Silver2),
                        position.count_hand(HandPiece::Knight2),
                        position.count_hand(HandPiece::Lance2),
                        position.count_hand(HandPiece::Pawn2),
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
        let position = game.get_board(PosNums::Current);
        let ply = game.history.ply;

        // 局面表示
        format!(
            "\
[{95} ply. {96} phase.]

         9    8    7    6    5    4    3    2    1
        +----+----+----+----+----+----+----+----+----+
▲       |{0:>4}|{1:>4}|{2:>4}|{3:>4}|{4:>4}|{5:>4}|{6:>4}|{7:>4}|{8:>4}| a    ▽
        +----+----+----+----+----+----+----+----+----+
R x{81:2}   |{9:>4}|{10:>4}|{11:>4}|{12:>4}|{13:>4}|{14:>4}|{15:>4}|{16:>4}|{17:>4}| b    r x{88:2}
        +----+----+----+----+----+----+----+----+----+
B x{82:2}   |{18:>4}|{19:>4}|{20:>4}|{21:>4}|{22:>4}|{23:>4}|{24:>4}|{25:>4}|{26:>4}| c    b x{89:2}
        +----+----+----+----+----+----+----+----+----+
G x{83:2}   |{27:>4}|{28:>4}|{29:>4}|{30:>4}|{31:>4}|{32:>4}|{33:>4}|{34:>4}|{35:>4}| d    g x{90:2}
        +----+----+----+----+----+----+----+----+----+
S x{84:2}   |{36:>4}|{37:>4}|{38:>4}|{39:>4}|{40:>4}|{41:>4}|{42:>4}|{43:>4}|{44:>4}| e    s x{91:2}
        +----+----+----+----+----+----+----+----+----+
N x{85:2}   |{45:>4}|{46:>4}|{47:>4}|{48:>4}|{49:>4}|{50:>4}|{51:>4}|{52:>4}|{53:>4}| f    n x{92:2}
        +----+----+----+----+----+----+----+----+----+
L x{86:2}   |{54:>4}|{55:>4}|{56:>4}|{57:>4}|{58:>4}|{59:>4}|{60:>4}|{61:>4}|{62:>4}| g    l x{93:2}
        +----+----+----+----+----+----+----+----+----+
P x{87:2}   |{63:>4}|{64:>4}|{65:>4}|{66:>4}|{67:>4}|{68:>4}|{69:>4}|{70:>4}|{71:>4}| h    p x{94:2}
        +----+----+----+----+----+----+----+----+----+
        |{72:>4}|{73:>4}|{74:>4}|{75:>4}|{76:>4}|{77:>4}|{78:>4}|{79:>4}|{80:>4}| i
        +----+----+----+----+----+----+----+----+----+\
        ",
            RestRoom::to_string2(position.get_control(phase, (9, 1))),
            RestRoom::to_string2(position.get_control(phase, (8, 1))),
            RestRoom::to_string2(position.get_control(phase, (7, 1))),
            RestRoom::to_string2(position.get_control(phase, (6, 1))),
            RestRoom::to_string2(position.get_control(phase, (5, 1))),
            RestRoom::to_string2(position.get_control(phase, (4, 1))),
            RestRoom::to_string2(position.get_control(phase, (3, 1))),
            RestRoom::to_string2(position.get_control(phase, (2, 1))),
            RestRoom::to_string2(position.get_control(phase, (1, 1))),
            RestRoom::to_string2(position.get_control(phase, (9, 2))),
            RestRoom::to_string2(position.get_control(phase, (8, 2))),
            RestRoom::to_string2(position.get_control(phase, (7, 2))),
            RestRoom::to_string2(position.get_control(phase, (6, 2))),
            RestRoom::to_string2(position.get_control(phase, (5, 2))),
            RestRoom::to_string2(position.get_control(phase, (4, 2))),
            RestRoom::to_string2(position.get_control(phase, (3, 2))),
            RestRoom::to_string2(position.get_control(phase, (2, 2))),
            RestRoom::to_string2(position.get_control(phase, (1, 2))),
            RestRoom::to_string2(position.get_control(phase, (9, 3))),
            RestRoom::to_string2(position.get_control(phase, (8, 3))),
            RestRoom::to_string2(position.get_control(phase, (7, 3))),
            RestRoom::to_string2(position.get_control(phase, (6, 3))),
            RestRoom::to_string2(position.get_control(phase, (5, 3))),
            RestRoom::to_string2(position.get_control(phase, (4, 3))),
            RestRoom::to_string2(position.get_control(phase, (3, 3))),
            RestRoom::to_string2(position.get_control(phase, (2, 3))),
            RestRoom::to_string2(position.get_control(phase, (1, 3))),
            RestRoom::to_string2(position.get_control(phase, (9, 4))),
            RestRoom::to_string2(position.get_control(phase, (8, 4))),
            RestRoom::to_string2(position.get_control(phase, (7, 4))),
            RestRoom::to_string2(position.get_control(phase, (6, 4))),
            RestRoom::to_string2(position.get_control(phase, (5, 4))),
            RestRoom::to_string2(position.get_control(phase, (4, 4))),
            RestRoom::to_string2(position.get_control(phase, (3, 4))),
            RestRoom::to_string2(position.get_control(phase, (2, 4))),
            RestRoom::to_string2(position.get_control(phase, (1, 4))),
            RestRoom::to_string2(position.get_control(phase, (9, 5))),
            RestRoom::to_string2(position.get_control(phase, (8, 5))),
            RestRoom::to_string2(position.get_control(phase, (7, 5))),
            RestRoom::to_string2(position.get_control(phase, (6, 5))),
            RestRoom::to_string2(position.get_control(phase, (5, 5))),
            RestRoom::to_string2(position.get_control(phase, (4, 5))),
            RestRoom::to_string2(position.get_control(phase, (3, 5))),
            RestRoom::to_string2(position.get_control(phase, (2, 5))),
            RestRoom::to_string2(position.get_control(phase, (1, 5))),
            RestRoom::to_string2(position.get_control(phase, (9, 6))),
            RestRoom::to_string2(position.get_control(phase, (8, 6))),
            RestRoom::to_string2(position.get_control(phase, (7, 6))),
            RestRoom::to_string2(position.get_control(phase, (6, 6))),
            RestRoom::to_string2(position.get_control(phase, (5, 6))),
            RestRoom::to_string2(position.get_control(phase, (4, 6))),
            RestRoom::to_string2(position.get_control(phase, (3, 6))),
            RestRoom::to_string2(position.get_control(phase, (2, 6))),
            RestRoom::to_string2(position.get_control(phase, (1, 6))),
            RestRoom::to_string2(position.get_control(phase, (9, 7))),
            RestRoom::to_string2(position.get_control(phase, (8, 7))),
            RestRoom::to_string2(position.get_control(phase, (7, 7))),
            RestRoom::to_string2(position.get_control(phase, (6, 7))),
            RestRoom::to_string2(position.get_control(phase, (5, 7))),
            RestRoom::to_string2(position.get_control(phase, (4, 7))),
            RestRoom::to_string2(position.get_control(phase, (3, 7))),
            RestRoom::to_string2(position.get_control(phase, (2, 7))),
            RestRoom::to_string2(position.get_control(phase, (1, 7))),
            RestRoom::to_string2(position.get_control(phase, (9, 8))),
            RestRoom::to_string2(position.get_control(phase, (8, 8))),
            RestRoom::to_string2(position.get_control(phase, (7, 8))),
            RestRoom::to_string2(position.get_control(phase, (6, 8))),
            RestRoom::to_string2(position.get_control(phase, (5, 8))),
            RestRoom::to_string2(position.get_control(phase, (4, 8))),
            RestRoom::to_string2(position.get_control(phase, (3, 8))),
            RestRoom::to_string2(position.get_control(phase, (2, 8))),
            RestRoom::to_string2(position.get_control(phase, (1, 8))),
            RestRoom::to_string2(position.get_control(phase, (9, 9))),
            RestRoom::to_string2(position.get_control(phase, (8, 9))),
            RestRoom::to_string2(position.get_control(phase, (7, 9))),
            RestRoom::to_string2(position.get_control(phase, (6, 9))),
            RestRoom::to_string2(position.get_control(phase, (5, 9))),
            RestRoom::to_string2(position.get_control(phase, (4, 9))),
            RestRoom::to_string2(position.get_control(phase, (3, 9))),
            RestRoom::to_string2(position.get_control(phase, (2, 9))),
            RestRoom::to_string2(position.get_control(phase, (1, 9))),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            position.count_hand(HandPiece::Rook1),
            position.count_hand(HandPiece::Bishop1),
            position.count_hand(HandPiece::Gold1),
            position.count_hand(HandPiece::Silver1),
            position.count_hand(HandPiece::Knight1),
            position.count_hand(HandPiece::Lance1),
            position.count_hand(HandPiece::Pawn1),
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            position.count_hand(HandPiece::Rook2),
            position.count_hand(HandPiece::Bishop2),
            position.count_hand(HandPiece::Gold2),
            position.count_hand(HandPiece::Silver2),
            position.count_hand(HandPiece::Knight2),
            position.count_hand(HandPiece::Lance2),
            position.count_hand(HandPiece::Pawn2),
            ply,
            phase,
        )
    }
}
*/
