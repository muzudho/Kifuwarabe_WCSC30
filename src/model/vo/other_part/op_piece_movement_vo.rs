//!
//! 駒の動き
//!

use super::super::game_part::gp_piece_type_vo::*;
use super::op_piece_direction_vo::*;

// 駒が戻る動き
#[allow(dead_code)]
pub struct PieceMovement {
    // 駒種類ごとに、駒の動きを保持。動ける方向は、駒ごとに可変長配列
    pub back: [[PieceDirection; KM_UGOKI_LN]; KMS_LN],
}
/**
 * 駒が戻る動き。投了図から現局面へ逆向きに指す思想。
 * [駒種類][9]
 *
 * （１）この表は、後手から盤面を見たものを想像する。
 * （２）後手から見て、普通に駒の動きが　登録されている。
 *       先手から見たとき、back （後ろ向きの動き）となる。
 */
pub const KM_UGOKI: PieceMovement = PieceMovement {
    back: [
        // 東,北東,北,北西,西,南西,南南西,南,南南東,南東,終わり
        /*ら  */
        [
            PieceDirection::E(false),
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::W(false),
            PieceDirection::SW(false),
            PieceDirection::S(false),
            PieceDirection::SE(false),
            PieceDirection::Owari,
        ],
        /*き  */
        [
            PieceDirection::E(true),
            PieceDirection::N(true),
            PieceDirection::W(true),
            PieceDirection::S(true),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ぞ  */
        [
            PieceDirection::NE(true),
            PieceDirection::NW(true),
            PieceDirection::SW(true),
            PieceDirection::SE(true),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*い  */
        [
            PieceDirection::E(false),
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::W(false),
            PieceDirection::S(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ね  */
        [
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::SW(false),
            PieceDirection::SE(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*う  */
        [
            PieceDirection::NNE,
            PieceDirection::NNW,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*し  */
        [
            PieceDirection::N(true),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ひ  */
        [
            PieceDirection::N(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ぱき*/
        [
            PieceDirection::E(true),
            PieceDirection::NE(false),
            PieceDirection::N(true),
            PieceDirection::NW(false),
            PieceDirection::W(true),
            PieceDirection::SW(false),
            PieceDirection::S(true),
            PieceDirection::SE(false),
            PieceDirection::Owari,
        ],
        /*ぱぞ*/
        [
            PieceDirection::E(false),
            PieceDirection::NE(true),
            PieceDirection::N(false),
            PieceDirection::NW(true),
            PieceDirection::W(false),
            PieceDirection::SW(true),
            PieceDirection::S(false),
            PieceDirection::SE(true),
            PieceDirection::Owari,
        ],
        /*ぱね*/
        [
            PieceDirection::E(false),
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::W(false),
            PieceDirection::S(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ぱう*/
        [
            PieceDirection::E(false),
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::W(false),
            PieceDirection::S(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ぱし*/
        [
            PieceDirection::E(false),
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::W(false),
            PieceDirection::S(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*ぱひ*/
        [
            PieceDirection::E(false),
            PieceDirection::NE(false),
            PieceDirection::N(false),
            PieceDirection::NW(false),
            PieceDirection::W(false),
            PieceDirection::S(false),
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*空升*/
        [
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
        /*終り*/
        [
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
            PieceDirection::Owari,
        ],
    ],
};
