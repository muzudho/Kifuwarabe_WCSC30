// 玉が移動したとき、敵の長い利きが当たっているかどうか。
// ただし、駒が動く前の position であることに注意してください。
use crate::entities::cosmic::smart::features::PieceType;
use crate::movegen::Direction;
use crate::movegen::Phase;
use crate::movegen::DIRECTIONS_SQ_FROM_FIRST;
use crate::movegen::DIRECTIONS_SQ_FROM_SECOND;
use crate::movegen::FILE_1;
use crate::movegen::FILE_10;
use crate::movegen::RANK_1;
use crate::movegen::RANK_10;
use crate::position::file;
use crate::position::position::Position;
use crate::position::rank;
use crate::position::Square;

pub fn king_is_adjacent_opponent_long_control(
    us: Phase,
    position_before_move: &Position,
    ksq_from: Square,
    ksq_to: Square,
    direction: Direction,
) -> bool {
    let d_sq = match us {
        Phase::First => DIRECTIONS_SQ_FROM_FIRST[direction as usize],
        Phase::Second => DIRECTIONS_SQ_FROM_SECOND[direction as usize],
    };
    let mut pinned = false;
    let mut pinned_opponent = false;
    let mut distance = 0;

    // 隣のマス
    let mut adjacent_sq = ksq_to as i8;
    loop {
        adjacent_sq += d_sq;

        let adjacent_file = file(adjacent_sq as u8);
        let adjacent_rank = rank(adjacent_sq as u8);
        if !(FILE_1 <= adjacent_file
            && adjacent_file < FILE_10
            && RANK_1 <= adjacent_rank
            && adjacent_rank < RANK_10)
        {
            break;
        }

        if adjacent_sq as u8 == ksq_from {
            // 動かす前の自玉があるマスは、何もないマスとして無視します
        } else if let Some(pc_ex) = position_before_move.piece_at_board(adjacent_sq as u8) {
            if us == pc_ex.piece.phase() {
                if pinned {
                    // 味方の駒が２つ有れば、ただちにディスカバード・アタックがくることは無い（＾～＾）
                    return false;
                } else {
                    // 味方の駒の１つ目は合い駒かも知れない（＾～＾）
                    // 影に 相手の長い利きの駒があるかも知れないから見逃すぜ（＾～＾）
                    pinned = true
                }
            } else {
                // 敵の駒なら
                if distance == 0 {
                    // 隣接する相手の駒の影に、相手の長い利きの駒があるかも知れないから見逃すぜ（＾～＾）
                    pinned_opponent = true;
                    continue;
                }

                match direction {
                    // 飛、香、竜
                    Direction::Top => match pc_ex.piece.type_() {
                        PieceType::R | PieceType::L | PieceType::PR => return true,
                        _ => {
                            // 相手の駒が２枚有れば、ただちにディスカバード・アタックがくることは無い（＾～＾）
                            if pinned_opponent {
                                return false;
                            }
                        }
                    },
                    // 飛、竜
                    Direction::Right | Direction::Left | Direction::Bottom => {
                        match pc_ex.piece.type_() {
                            PieceType::R | PieceType::PR => return true,
                            _ => {
                                // 相手の駒が２枚有れば、ただちにディスカバード・アタックがくることは無い（＾～＾）
                                if pinned_opponent {
                                    return false;
                                }
                            }
                        }
                    }
                    // 角、馬
                    Direction::TopRight
                    | Direction::TopLeft
                    | Direction::BottomLeft
                    | Direction::BottomRight => match pc_ex.piece.type_() {
                        PieceType::B | PieceType::PB => return true,
                        _ => {
                            // 相手の駒が２枚有れば、ただちにディスカバード・アタックがくることは無い（＾～＾）
                            if pinned_opponent {
                                return false;
                            }
                        }
                    },
                    // ここ（桂馬の動き）を通るとは想定していないぜ（＾～＾）
                    Direction::TopRightKnight | Direction::TopLeftKnight => {}
                };
            }
        }

        distance += 1;
    }

    false
}
