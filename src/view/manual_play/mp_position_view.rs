use super::super::super::model::vo::game_part::gp_piece_vo::GPPieceVo;
use super::super::super::model::vo::game_part::gp_square_vo::*;
use super::super::super::model::vo::other_part::op_misc_vo::*;
use super::super::super::model::vo::other_part::op_person_vo::Person;
use crate::model::dto::main_loop::ml_universe_dto::MLUniverseDto;

/// 表示
///
/// 後手から見た盤を表示するぜ☆（＾～＾）
/// デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
pub fn print_pos(ml_universe_dto: &MLUniverseDto, num: &KyNums) -> String {
    let cur_pos = match *num {
        KyNums::Current => ml_universe_dto.get_search_part().get_current_position(),
        KyNums::Start => ml_universe_dto.get_starting_position(),
    };

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
        cur_pos.get_piece_by_square(&Square::from_umasu(19)),
        cur_pos.get_piece_by_square(&Square::from_umasu(29)),
        cur_pos.get_piece_by_square(&Square::from_umasu(39)),
        cur_pos.get_piece_by_square(&Square::from_umasu(49)),
        cur_pos.get_piece_by_square(&Square::from_umasu(59)),
        cur_pos.get_piece_by_square(&Square::from_umasu(69)),
        cur_pos.get_piece_by_square(&Square::from_umasu(79)),
        cur_pos.get_piece_by_square(&Square::from_umasu(89)),
        cur_pos.get_piece_by_square(&Square::from_umasu(99)),
        cur_pos.get_piece_by_square(&Square::from_umasu(18)),
        cur_pos.get_piece_by_square(&Square::from_umasu(28)),
        cur_pos.get_piece_by_square(&Square::from_umasu(38)),
        cur_pos.get_piece_by_square(&Square::from_umasu(48)),
        cur_pos.get_piece_by_square(&Square::from_umasu(58)),
        cur_pos.get_piece_by_square(&Square::from_umasu(68)),
        cur_pos.get_piece_by_square(&Square::from_umasu(78)),
        cur_pos.get_piece_by_square(&Square::from_umasu(88)),
        cur_pos.get_piece_by_square(&Square::from_umasu(98)),
        cur_pos.get_piece_by_square(&Square::from_umasu(17)),
        cur_pos.get_piece_by_square(&Square::from_umasu(27)),
        cur_pos.get_piece_by_square(&Square::from_umasu(37)),
        cur_pos.get_piece_by_square(&Square::from_umasu(47)),
        cur_pos.get_piece_by_square(&Square::from_umasu(57)),
        cur_pos.get_piece_by_square(&Square::from_umasu(67)),
        cur_pos.get_piece_by_square(&Square::from_umasu(77)),
        cur_pos.get_piece_by_square(&Square::from_umasu(87)),
        cur_pos.get_piece_by_square(&Square::from_umasu(97)),
        cur_pos.get_piece_by_square(&Square::from_umasu(16)),
        cur_pos.get_piece_by_square(&Square::from_umasu(26)),
        cur_pos.get_piece_by_square(&Square::from_umasu(36)),
        cur_pos.get_piece_by_square(&Square::from_umasu(46)),
        cur_pos.get_piece_by_square(&Square::from_umasu(56)),
        cur_pos.get_piece_by_square(&Square::from_umasu(66)),
        cur_pos.get_piece_by_square(&Square::from_umasu(76)),
        cur_pos.get_piece_by_square(&Square::from_umasu(86)),
        cur_pos.get_piece_by_square(&Square::from_umasu(96)),
        cur_pos.get_piece_by_square(&Square::from_umasu(15)),
        cur_pos.get_piece_by_square(&Square::from_umasu(25)),
        cur_pos.get_piece_by_square(&Square::from_umasu(35)),
        cur_pos.get_piece_by_square(&Square::from_umasu(45)),
        cur_pos.get_piece_by_square(&Square::from_umasu(55)),
        cur_pos.get_piece_by_square(&Square::from_umasu(65)),
        cur_pos.get_piece_by_square(&Square::from_umasu(75)),
        cur_pos.get_piece_by_square(&Square::from_umasu(85)),
        cur_pos.get_piece_by_square(&Square::from_umasu(95)),
        cur_pos.get_piece_by_square(&Square::from_umasu(14)),
        cur_pos.get_piece_by_square(&Square::from_umasu(24)),
        cur_pos.get_piece_by_square(&Square::from_umasu(34)),
        cur_pos.get_piece_by_square(&Square::from_umasu(44)),
        cur_pos.get_piece_by_square(&Square::from_umasu(54)),
        cur_pos.get_piece_by_square(&Square::from_umasu(64)),
        cur_pos.get_piece_by_square(&Square::from_umasu(74)),
        cur_pos.get_piece_by_square(&Square::from_umasu(84)),
        cur_pos.get_piece_by_square(&Square::from_umasu(94)),
        cur_pos.get_piece_by_square(&Square::from_umasu(13)),
        cur_pos.get_piece_by_square(&Square::from_umasu(23)),
        cur_pos.get_piece_by_square(&Square::from_umasu(33)),
        cur_pos.get_piece_by_square(&Square::from_umasu(43)),
        cur_pos.get_piece_by_square(&Square::from_umasu(53)),
        cur_pos.get_piece_by_square(&Square::from_umasu(63)),
        cur_pos.get_piece_by_square(&Square::from_umasu(73)),
        cur_pos.get_piece_by_square(&Square::from_umasu(83)),
        cur_pos.get_piece_by_square(&Square::from_umasu(93)),
        cur_pos.get_piece_by_square(&Square::from_umasu(12)),
        cur_pos.get_piece_by_square(&Square::from_umasu(22)),
        cur_pos.get_piece_by_square(&Square::from_umasu(32)),
        cur_pos.get_piece_by_square(&Square::from_umasu(42)),
        cur_pos.get_piece_by_square(&Square::from_umasu(52)),
        cur_pos.get_piece_by_square(&Square::from_umasu(62)),
        cur_pos.get_piece_by_square(&Square::from_umasu(72)),
        cur_pos.get_piece_by_square(&Square::from_umasu(82)),
        cur_pos.get_piece_by_square(&Square::from_umasu(92)),
        cur_pos.get_piece_by_square(&Square::from_umasu(11)),
        cur_pos.get_piece_by_square(&Square::from_umasu(21)),
        cur_pos.get_piece_by_square(&Square::from_umasu(31)),
        cur_pos.get_piece_by_square(&Square::from_umasu(41)),
        cur_pos.get_piece_by_square(&Square::from_umasu(51)),
        cur_pos.get_piece_by_square(&Square::from_umasu(61)),
        cur_pos.get_piece_by_square(&Square::from_umasu(71)),
        cur_pos.get_piece_by_square(&Square::from_umasu(81)),
        cur_pos.get_piece_by_square(&Square::from_umasu(91)),
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
        ml_universe_dto.get_search_part().get_ply(),
        ml_universe_dto.get_search_part().get_phase(&Person::Friend),
        ml_universe_dto.count_same_ky()
    )
}
