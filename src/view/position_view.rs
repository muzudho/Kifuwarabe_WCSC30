use super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::model::vo::game_part::gp_square_vo::*;
use crate::model::dto::search_part::board::Board;
use crate::model::vo::game_part::gp_phase_vo::Phase;

pub struct PositionView {}
impl PositionView {
    /// 表示
    ///
    /// 後手から見た盤を表示するぜ☆（＾～＾）
    /// デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
    pub fn to_string(cur_pos: &Board, ply: i16, phase: Phase, same_pos_count: i8) -> String {
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
            cur_pos.get_piece_by_square(&Square::from_usquare(19)),
            cur_pos.get_piece_by_square(&Square::from_usquare(29)),
            cur_pos.get_piece_by_square(&Square::from_usquare(39)),
            cur_pos.get_piece_by_square(&Square::from_usquare(49)),
            cur_pos.get_piece_by_square(&Square::from_usquare(59)),
            cur_pos.get_piece_by_square(&Square::from_usquare(69)),
            cur_pos.get_piece_by_square(&Square::from_usquare(79)),
            cur_pos.get_piece_by_square(&Square::from_usquare(89)),
            cur_pos.get_piece_by_square(&Square::from_usquare(99)),
            cur_pos.get_piece_by_square(&Square::from_usquare(18)),
            cur_pos.get_piece_by_square(&Square::from_usquare(28)),
            cur_pos.get_piece_by_square(&Square::from_usquare(38)),
            cur_pos.get_piece_by_square(&Square::from_usquare(48)),
            cur_pos.get_piece_by_square(&Square::from_usquare(58)),
            cur_pos.get_piece_by_square(&Square::from_usquare(68)),
            cur_pos.get_piece_by_square(&Square::from_usquare(78)),
            cur_pos.get_piece_by_square(&Square::from_usquare(88)),
            cur_pos.get_piece_by_square(&Square::from_usquare(98)),
            cur_pos.get_piece_by_square(&Square::from_usquare(17)),
            cur_pos.get_piece_by_square(&Square::from_usquare(27)),
            cur_pos.get_piece_by_square(&Square::from_usquare(37)),
            cur_pos.get_piece_by_square(&Square::from_usquare(47)),
            cur_pos.get_piece_by_square(&Square::from_usquare(57)),
            cur_pos.get_piece_by_square(&Square::from_usquare(67)),
            cur_pos.get_piece_by_square(&Square::from_usquare(77)),
            cur_pos.get_piece_by_square(&Square::from_usquare(87)),
            cur_pos.get_piece_by_square(&Square::from_usquare(97)),
            cur_pos.get_piece_by_square(&Square::from_usquare(16)),
            cur_pos.get_piece_by_square(&Square::from_usquare(26)),
            cur_pos.get_piece_by_square(&Square::from_usquare(36)),
            cur_pos.get_piece_by_square(&Square::from_usquare(46)),
            cur_pos.get_piece_by_square(&Square::from_usquare(56)),
            cur_pos.get_piece_by_square(&Square::from_usquare(66)),
            cur_pos.get_piece_by_square(&Square::from_usquare(76)),
            cur_pos.get_piece_by_square(&Square::from_usquare(86)),
            cur_pos.get_piece_by_square(&Square::from_usquare(96)),
            cur_pos.get_piece_by_square(&Square::from_usquare(15)),
            cur_pos.get_piece_by_square(&Square::from_usquare(25)),
            cur_pos.get_piece_by_square(&Square::from_usquare(35)),
            cur_pos.get_piece_by_square(&Square::from_usquare(45)),
            cur_pos.get_piece_by_square(&Square::from_usquare(55)),
            cur_pos.get_piece_by_square(&Square::from_usquare(65)),
            cur_pos.get_piece_by_square(&Square::from_usquare(75)),
            cur_pos.get_piece_by_square(&Square::from_usquare(85)),
            cur_pos.get_piece_by_square(&Square::from_usquare(95)),
            cur_pos.get_piece_by_square(&Square::from_usquare(14)),
            cur_pos.get_piece_by_square(&Square::from_usquare(24)),
            cur_pos.get_piece_by_square(&Square::from_usquare(34)),
            cur_pos.get_piece_by_square(&Square::from_usquare(44)),
            cur_pos.get_piece_by_square(&Square::from_usquare(54)),
            cur_pos.get_piece_by_square(&Square::from_usquare(64)),
            cur_pos.get_piece_by_square(&Square::from_usquare(74)),
            cur_pos.get_piece_by_square(&Square::from_usquare(84)),
            cur_pos.get_piece_by_square(&Square::from_usquare(94)),
            cur_pos.get_piece_by_square(&Square::from_usquare(13)),
            cur_pos.get_piece_by_square(&Square::from_usquare(23)),
            cur_pos.get_piece_by_square(&Square::from_usquare(33)),
            cur_pos.get_piece_by_square(&Square::from_usquare(43)),
            cur_pos.get_piece_by_square(&Square::from_usquare(53)),
            cur_pos.get_piece_by_square(&Square::from_usquare(63)),
            cur_pos.get_piece_by_square(&Square::from_usquare(73)),
            cur_pos.get_piece_by_square(&Square::from_usquare(83)),
            cur_pos.get_piece_by_square(&Square::from_usquare(93)),
            cur_pos.get_piece_by_square(&Square::from_usquare(12)),
            cur_pos.get_piece_by_square(&Square::from_usquare(22)),
            cur_pos.get_piece_by_square(&Square::from_usquare(32)),
            cur_pos.get_piece_by_square(&Square::from_usquare(42)),
            cur_pos.get_piece_by_square(&Square::from_usquare(52)),
            cur_pos.get_piece_by_square(&Square::from_usquare(62)),
            cur_pos.get_piece_by_square(&Square::from_usquare(72)),
            cur_pos.get_piece_by_square(&Square::from_usquare(82)),
            cur_pos.get_piece_by_square(&Square::from_usquare(92)),
            cur_pos.get_piece_by_square(&Square::from_usquare(11)),
            cur_pos.get_piece_by_square(&Square::from_usquare(21)),
            cur_pos.get_piece_by_square(&Square::from_usquare(31)),
            cur_pos.get_piece_by_square(&Square::from_usquare(41)),
            cur_pos.get_piece_by_square(&Square::from_usquare(51)),
            cur_pos.get_piece_by_square(&Square::from_usquare(61)),
            cur_pos.get_piece_by_square(&Square::from_usquare(71)),
            cur_pos.get_piece_by_square(&Square::from_usquare(81)),
            cur_pos.get_piece_by_square(&Square::from_usquare(91)),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            cur_pos.hand[GPPieceVo::Rook1 as usize],
            cur_pos.hand[GPPieceVo::Bishop1 as usize],
            cur_pos.hand[GPPieceVo::Gold1 as usize],
            cur_pos.hand[GPPieceVo::Silver1 as usize],
            cur_pos.hand[GPPieceVo::Knight1 as usize],
            cur_pos.hand[GPPieceVo::Lance1 as usize],
            cur_pos.hand[GPPieceVo::Pawn1 as usize],
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            cur_pos.hand[GPPieceVo::Rook2 as usize],
            cur_pos.hand[GPPieceVo::Bishop2 as usize],
            cur_pos.hand[GPPieceVo::Gold2 as usize],
            cur_pos.hand[GPPieceVo::Silver2 as usize],
            cur_pos.hand[GPPieceVo::Knight2 as usize],
            cur_pos.hand[GPPieceVo::Lance2 as usize],
            cur_pos.hand[GPPieceVo::Pawn2 as usize],
            ply,
            phase,
            same_pos_count
        )
    }
}
