use crate::cosmic::shogi::playing::{Game, PosNums};
use crate::cosmic::shogi::state::Person;
use crate::cosmic::smart::square::*;
use crate::cosmic::toy_box::{Piece, PIECE_WHITE_SPACE};

pub struct GameView {}
impl GameView {
        fn to_string2(piece_o: Option<Piece>) -> String {
                if let Some(piece) = piece_o {
                        format!("{}", piece)
                } else {
                        PIECE_WHITE_SPACE.to_string()
                }
        }
        /// 表示
        pub fn to_string(game: &Game, pos_nums: &PosNums) -> String {
                let board = game.get_board(pos_nums);
                let ply = game.history.ply;
                let phase = game.history.get_phase(&Person::Friend);
                let same_pos_count = game.count_same_ky();

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
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(91))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(81))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(71))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(61))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(51))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(41))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(31))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(21))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(11))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(92))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(82))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(72))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(62))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(52))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(42))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(32))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(22))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(12))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(93))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(83))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(73))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(63))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(53))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(43))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(33))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(23))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(13))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(94))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(84))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(74))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(64))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(54))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(44))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(34))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(24))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(14))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(95))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(85))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(75))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(65))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(55))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(45))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(35))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(25))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(15))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(96))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(86))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(76))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(66))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(56))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(46))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(36))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(26))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(16))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(97))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(87))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(77))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(67))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(57))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(47))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(37))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(27))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(17))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(98))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(88))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(78))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(68))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(58))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(48))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(38))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(28))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(18))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(99))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(89))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(79))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(69))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(59))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(49))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(39))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(29))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_address(19))),
                        //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
                        board.hand[Piece::Rook1 as usize],
                        board.hand[Piece::Bishop1 as usize],
                        board.hand[Piece::Gold1 as usize],
                        board.hand[Piece::Silver1 as usize],
                        board.hand[Piece::Knight1 as usize],
                        board.hand[Piece::Lance1 as usize],
                        board.hand[Piece::Pawn1 as usize],
                        //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
                        board.hand[Piece::Rook2 as usize],
                        board.hand[Piece::Bishop2 as usize],
                        board.hand[Piece::Gold2 as usize],
                        board.hand[Piece::Silver2 as usize],
                        board.hand[Piece::Knight2 as usize],
                        board.hand[Piece::Lance2 as usize],
                        board.hand[Piece::Pawn2 as usize],
                        ply,
                        phase,
                        same_pos_count
                )
        }
}
