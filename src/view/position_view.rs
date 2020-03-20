use super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::model::vo::game_part::gp_square_vo::*;
use crate::model::universe::Universe;
use crate::model::vo::other_part::op_misc_vo::PosNums;
use crate::model::vo::other_part::op_person_vo::Person;

pub struct PositionView {}
impl PositionView {
    /// 表示
    pub fn to_string(universe: &Universe, pos_nums: &PosNums) -> String {
        let board = universe.get_board(pos_nums);
        let ply = universe.get_position().get_ply();
        let phase = universe.get_position().get_phase(&Person::Friend);
        let same_pos_count = universe.count_same_ky();

        // 局面表示
        format!(
            "\
[{95} ply. {96} phase. {97} repeats.]

           +----+----+----+----+----+----+----+----+----+
        i9 |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|   △
           +----+----+----+----+----+----+----+----+----+
P x{87:2}   h8 |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|   p x{94:2}
           +----+----+----+----+----+----+----+----+----+
L x{86:2}   g7 |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}|   l x{93:2}
           +----+----+----+----+----+----+----+----+----+
N x{85:2}   f6 |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}|   n x{92:2}
           +----+----+----+----+----+----+----+----+----+
S x{84:2}   e5 |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}|   s x{91:2}
           +----+----+----+----+----+----+----+----+----+
G x{83:2}   d4 |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}|   g x{90:2}
           +----+----+----+----+----+----+----+----+----+
B x{82:2}   c3 |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}|   b x{89:2}
           +----+----+----+----+----+----+----+----+----+
R x{81:2}   b2 |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}|   r x{88:2}
           +----+----+----+----+----+----+----+----+----+
▼       a1 |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}|
           +----+----+----+----+----+----+----+----+----+
            1    2    3    4    5    6    7    8    9\
",
            board.get_piece_by_square(&Square::from_usquare(19)),
            board.get_piece_by_square(&Square::from_usquare(29)),
            board.get_piece_by_square(&Square::from_usquare(39)),
            board.get_piece_by_square(&Square::from_usquare(49)),
            board.get_piece_by_square(&Square::from_usquare(59)),
            board.get_piece_by_square(&Square::from_usquare(69)),
            board.get_piece_by_square(&Square::from_usquare(79)),
            board.get_piece_by_square(&Square::from_usquare(89)),
            board.get_piece_by_square(&Square::from_usquare(99)),
            board.get_piece_by_square(&Square::from_usquare(18)),
            board.get_piece_by_square(&Square::from_usquare(28)),
            board.get_piece_by_square(&Square::from_usquare(38)),
            board.get_piece_by_square(&Square::from_usquare(48)),
            board.get_piece_by_square(&Square::from_usquare(58)),
            board.get_piece_by_square(&Square::from_usquare(68)),
            board.get_piece_by_square(&Square::from_usquare(78)),
            board.get_piece_by_square(&Square::from_usquare(88)),
            board.get_piece_by_square(&Square::from_usquare(98)),
            board.get_piece_by_square(&Square::from_usquare(17)),
            board.get_piece_by_square(&Square::from_usquare(27)),
            board.get_piece_by_square(&Square::from_usquare(37)),
            board.get_piece_by_square(&Square::from_usquare(47)),
            board.get_piece_by_square(&Square::from_usquare(57)),
            board.get_piece_by_square(&Square::from_usquare(67)),
            board.get_piece_by_square(&Square::from_usquare(77)),
            board.get_piece_by_square(&Square::from_usquare(87)),
            board.get_piece_by_square(&Square::from_usquare(97)),
            board.get_piece_by_square(&Square::from_usquare(16)),
            board.get_piece_by_square(&Square::from_usquare(26)),
            board.get_piece_by_square(&Square::from_usquare(36)),
            board.get_piece_by_square(&Square::from_usquare(46)),
            board.get_piece_by_square(&Square::from_usquare(56)),
            board.get_piece_by_square(&Square::from_usquare(66)),
            board.get_piece_by_square(&Square::from_usquare(76)),
            board.get_piece_by_square(&Square::from_usquare(86)),
            board.get_piece_by_square(&Square::from_usquare(96)),
            board.get_piece_by_square(&Square::from_usquare(15)),
            board.get_piece_by_square(&Square::from_usquare(25)),
            board.get_piece_by_square(&Square::from_usquare(35)),
            board.get_piece_by_square(&Square::from_usquare(45)),
            board.get_piece_by_square(&Square::from_usquare(55)),
            board.get_piece_by_square(&Square::from_usquare(65)),
            board.get_piece_by_square(&Square::from_usquare(75)),
            board.get_piece_by_square(&Square::from_usquare(85)),
            board.get_piece_by_square(&Square::from_usquare(95)),
            board.get_piece_by_square(&Square::from_usquare(14)),
            board.get_piece_by_square(&Square::from_usquare(24)),
            board.get_piece_by_square(&Square::from_usquare(34)),
            board.get_piece_by_square(&Square::from_usquare(44)),
            board.get_piece_by_square(&Square::from_usquare(54)),
            board.get_piece_by_square(&Square::from_usquare(64)),
            board.get_piece_by_square(&Square::from_usquare(74)),
            board.get_piece_by_square(&Square::from_usquare(84)),
            board.get_piece_by_square(&Square::from_usquare(94)),
            board.get_piece_by_square(&Square::from_usquare(13)),
            board.get_piece_by_square(&Square::from_usquare(23)),
            board.get_piece_by_square(&Square::from_usquare(33)),
            board.get_piece_by_square(&Square::from_usquare(43)),
            board.get_piece_by_square(&Square::from_usquare(53)),
            board.get_piece_by_square(&Square::from_usquare(63)),
            board.get_piece_by_square(&Square::from_usquare(73)),
            board.get_piece_by_square(&Square::from_usquare(83)),
            board.get_piece_by_square(&Square::from_usquare(93)),
            board.get_piece_by_square(&Square::from_usquare(12)),
            board.get_piece_by_square(&Square::from_usquare(22)),
            board.get_piece_by_square(&Square::from_usquare(32)),
            board.get_piece_by_square(&Square::from_usquare(42)),
            board.get_piece_by_square(&Square::from_usquare(52)),
            board.get_piece_by_square(&Square::from_usquare(62)),
            board.get_piece_by_square(&Square::from_usquare(72)),
            board.get_piece_by_square(&Square::from_usquare(82)),
            board.get_piece_by_square(&Square::from_usquare(92)),
            board.get_piece_by_square(&Square::from_usquare(11)),
            board.get_piece_by_square(&Square::from_usquare(21)),
            board.get_piece_by_square(&Square::from_usquare(31)),
            board.get_piece_by_square(&Square::from_usquare(41)),
            board.get_piece_by_square(&Square::from_usquare(51)),
            board.get_piece_by_square(&Square::from_usquare(61)),
            board.get_piece_by_square(&Square::from_usquare(71)),
            board.get_piece_by_square(&Square::from_usquare(81)),
            board.get_piece_by_square(&Square::from_usquare(91)),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            board.hand[GPPieceVo::Rook1 as usize],
            board.hand[GPPieceVo::Bishop1 as usize],
            board.hand[GPPieceVo::Gold1 as usize],
            board.hand[GPPieceVo::Silver1 as usize],
            board.hand[GPPieceVo::Knight1 as usize],
            board.hand[GPPieceVo::Lance1 as usize],
            board.hand[GPPieceVo::Pawn1 as usize],
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            board.hand[GPPieceVo::Rook2 as usize],
            board.hand[GPPieceVo::Bishop2 as usize],
            board.hand[GPPieceVo::Gold2 as usize],
            board.hand[GPPieceVo::Silver2 as usize],
            board.hand[GPPieceVo::Knight2 as usize],
            board.hand[GPPieceVo::Lance2 as usize],
            board.hand[GPPieceVo::Pawn2 as usize],
            ply,
            phase,
            same_pos_count
        )
    }
}
