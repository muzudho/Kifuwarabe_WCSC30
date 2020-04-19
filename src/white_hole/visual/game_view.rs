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
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(91))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(81))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(71))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(61))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(51))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(41))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(31))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(21))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(11))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(92))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(82))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(72))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(62))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(52))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(42))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(32))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(22))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(12))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(93))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(83))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(73))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(63))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(53))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(43))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(33))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(23))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(13))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(94))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(84))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(74))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(64))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(54))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(44))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(34))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(24))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(14))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(95))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(85))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(75))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(65))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(55))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(45))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(35))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(25))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(15))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(96))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(86))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(76))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(66))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(56))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(46))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(36))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(26))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(16))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(97))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(87))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(77))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(67))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(57))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(47))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(37))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(27))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(17))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(98))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(88))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(78))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(68))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(58))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(48))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(38))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(28))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(18))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(99))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(89))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(79))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(69))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(59))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(49))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(39))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(29))),
                        GameView::to_string2(board.piece_at(&AbsoluteAddress::from_number(19))),
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
