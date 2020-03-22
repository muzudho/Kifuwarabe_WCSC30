use crate::model::univ::gam::misc::person::Person;
use crate::model::univ::gam::misc::piece::Piece;
use crate::model::univ::gam::misc::piece::PIECE_WHITE_SPACE;
use crate::model::univ::gam::misc::square::*;
use crate::model::univ::game::Game;
use crate::model::univ::game::*;

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
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(91))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(81))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(71))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(61))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(51))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(41))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(31))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(21))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(11))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(92))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(82))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(72))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(62))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(52))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(42))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(32))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(22))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(12))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(93))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(83))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(73))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(63))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(53))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(43))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(33))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(23))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(13))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(94))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(84))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(74))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(64))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(54))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(44))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(34))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(24))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(14))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(95))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(85))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(75))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(65))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(55))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(45))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(35))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(25))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(15))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(96))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(86))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(76))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(66))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(56))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(46))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(36))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(26))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(16))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(97))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(87))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(77))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(67))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(57))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(47))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(37))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(27))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(17))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(98))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(88))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(78))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(68))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(58))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(48))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(38))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(28))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(18))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(99))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(89))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(79))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(69))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(59))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(49))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(39))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(29))),
                        GameView::to_string2(board.get_piece_by_square(&Square::from_usquare(19))),
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
