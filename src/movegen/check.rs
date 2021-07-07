// 玉が移動したとき、敵の長い利きが当たっているかどうか。
// ただし、駒が動く前の position であることに注意してください。
use crate::entities::cosmic::smart::features::PieceType;
use crate::movegen::Beam;
use crate::movegen::Direction;
use crate::movegen::Phase;
use crate::position::position::Position;
use crate::position::Square;
use crate::position::FILE_1;
use crate::position::FILE_10;
use crate::position::RANK_1;
use crate::position::RANK_10;

/// 先手から見た向き
/// 筋は -1 すると右（＾～＾）
/// 段は -1 すると上（＾～＾）
const DIRECTIONS_SQ_FROM_FIRST: [i8; 10] = [
    -10, // 右方向
    -11, // 右上方向
    -1,  // 上方向
    9,   // 左上方向
    10,  // 左方向
    11,  // 左下方向
    1,   // 下方向
    -9,  // 右下方向
    -12, // 桂右
    8,   // 桂左
];

/// 後手から見た向き
/// 筋は 1 すると右（＾～＾）
/// 段は 1 すると上（＾～＾）
const DIRECTIONS_SQ_FROM_SECOND: [i8; 10] = [
    10,  // 右方向
    11,  // 右上方向
    1,   // 上方向
    -9,  // 左上方向
    -10, // 左方向
    -11, // 左下方向
    -1,  // 下方向
    9,   // 右下方向
    -8,  // 桂右
    12,  // 桂左
];

/// 長い利きで王手されているか
pub fn is_long_check(
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
    let mut adjacent_sq = ksq_to.number() as i8;
    loop {
        adjacent_sq += d_sq;

        let adjacent_file = Square::new(adjacent_sq as u8).file();
        let adjacent_rank = Square::new(adjacent_sq as u8).rank();
        if !(FILE_1 <= adjacent_file
            && adjacent_file < FILE_10
            && RANK_1 <= adjacent_rank
            && adjacent_rank < RANK_10)
        {
            break;
        }

        if adjacent_sq as u8 == ksq_from.number() {
            // 動かす前の自玉があるマスは、何もないマスとして無視します
        } else if let Some(pc_ex) =
            position_before_move.piece_at_board(Square::new(adjacent_sq as u8))
        {
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

// 隣接する敵の１マスの利きが利いているかどうか（桂も含む）
pub fn is_adjacent_check(
    us: Phase,
    position: &Position,
    ksq_to: Square,
    direction: Direction,
) -> bool {
    let d_sq = match us {
        Phase::First => DIRECTIONS_SQ_FROM_FIRST[direction as usize],
        Phase::Second => DIRECTIONS_SQ_FROM_SECOND[direction as usize],
    };

    // 隣のマス
    let adjacent_sq = (ksq_to.number() as i8 + d_sq) as u8;
    // Beam::shoot(&format!(
    //     "is_adjacent_check d_file={} d_rank={} adjacent_sq={}",
    //     d_file, d_rank, adjacent_sq
    // ));

    if Square::new(adjacent_sq).is_board() {
        if let Some(pc_ex) = position.piece_at_board(Square::new(adjacent_sq)) {
            if us != pc_ex.piece.phase() {
                // 敵の駒なら
                match direction {
                    Direction::Right | Direction::Left => match pc_ex.piece.type_() {
                        PieceType::K
                        | PieceType::R
                        | PieceType::G
                        | PieceType::PR
                        | PieceType::PB
                        | PieceType::PS
                        | PieceType::PN
                        | PieceType::PL
                        | PieceType::PP => return true,
                        _ => {}
                    },
                    Direction::TopRight | Direction::TopLeft => match pc_ex.piece.type_() {
                        PieceType::K
                        | PieceType::B
                        | PieceType::G
                        | PieceType::S
                        | PieceType::PR
                        | PieceType::PB
                        | PieceType::PS
                        | PieceType::PN
                        | PieceType::PL
                        | PieceType::PP => return true,
                        _ => {}
                    },
                    Direction::Top => match pc_ex.piece.type_() {
                        PieceType::K
                        | PieceType::R
                        | PieceType::G
                        | PieceType::S
                        | PieceType::L
                        | PieceType::P
                        | PieceType::PR
                        | PieceType::PB
                        | PieceType::PS
                        | PieceType::PN
                        | PieceType::PL
                        | PieceType::PP => return true,
                        _ => {}
                    },
                    Direction::BottomLeft | Direction::BottomRight => match pc_ex.piece.type_() {
                        PieceType::K
                        | PieceType::B
                        | PieceType::S
                        | PieceType::PR
                        | PieceType::PB => return true,
                        _ => {}
                    },
                    Direction::Bottom => match pc_ex.piece.type_() {
                        PieceType::K
                        | PieceType::R
                        | PieceType::G
                        | PieceType::PR
                        | PieceType::PB
                        | PieceType::PS
                        | PieceType::PN
                        | PieceType::PL
                        | PieceType::PP => return true,
                        _ => {}
                    },
                    // 桂馬
                    Direction::TopRightKnight | Direction::TopLeftKnight => {
                        match pc_ex.piece.type_() {
                            PieceType::N => return true,
                            _ => {}
                        }
                    }
                };
            }
        }
    }
    false
}

/// # Arguments
///
/// * `sq_list` - 玉を含まず、チェッカーを含む１列のマスのリストが追加されます
///
/// # Returns
///
/// 合い駒のマス, ピンしてる駒のマス、チェッカーのマス
pub fn get_check(
    us: Phase,
    position: &Position,
    ksq: Square,
    direction: Direction,
    sq_list: &mut Vec<Square>,
    is_debug: bool,
) -> (Option<Square>, Option<Square>, Option<Square>) {
    // 先後の違いを吸収
    let d_sq = match us {
        Phase::First => DIRECTIONS_SQ_FROM_FIRST[direction as usize],
        Phase::Second => DIRECTIONS_SQ_FROM_SECOND[direction as usize],
    };
    if is_debug {
        Beam::shoot(&format!(
            "# get_check us={} ksq={} d_sq={} direction={:?}",
            us, ksq, d_sq, direction
        ));
    }

    let mut sq = Square::new((ksq.number() as i8 + d_sq) as u8);
    let mut pinned: Option<Square> = None; // 合い駒か、ただの自駒
    let mut pin_head: Option<Square> = None; // ピンしてる駒
    let mut checker: Option<Square> = None; // チェック駒
    let mut interval = 0;
    while FILE_1 <= sq.file() && sq.file() < FILE_10 && RANK_1 <= sq.rank() && sq.rank() < RANK_10 {
        sq_list.push(sq);

        if let Some(pc_ex) = position.piece_at_board(sq) {
            if us == pc_ex.piece.phase() {
                // 合い駒か、ただの自駒か
                if let None = pinned {
                    // とりあえず 合い駒 候補
                    pinned = Some(sq);
                    if is_debug {
                        Beam::shoot(&format!("# get_check sq={} (Pinned?)", sq));
                    }
                } else {
                    // 味方の駒が２枚あれば長い利きは当たっていません
                    // ループ終了
                    if is_debug {
                        Beam::shoot(&format!("# get_check sq={} (End)", sq));
                    }
                    interval += 1;
                    break;
                }
            } else {
                // 敵駒
                if interval == 0 {
                    // 隣接する敵駒はチェッカー（＾～＾）
                    checker = match direction {
                        Direction::Right | Direction::Left => match pc_ex.piece.type_() {
                            PieceType::K
                            | PieceType::R
                            | PieceType::G
                            | PieceType::PR
                            | PieceType::PB
                            | PieceType::PS
                            | PieceType::PN
                            | PieceType::PL
                            | PieceType::PP => Some(sq),
                            _ => None,
                        },
                        Direction::TopRight | Direction::TopLeft => match pc_ex.piece.type_() {
                            PieceType::K
                            | PieceType::B
                            | PieceType::G
                            | PieceType::S
                            | PieceType::PR
                            | PieceType::PB
                            | PieceType::PS
                            | PieceType::PN
                            | PieceType::PL
                            | PieceType::PP => Some(sq),
                            _ => None,
                        },
                        Direction::Top => match pc_ex.piece.type_() {
                            PieceType::K
                            | PieceType::R
                            | PieceType::G
                            | PieceType::S
                            | PieceType::L
                            | PieceType::P
                            | PieceType::PR
                            | PieceType::PB
                            | PieceType::PS
                            | PieceType::PN
                            | PieceType::PL
                            | PieceType::PP => Some(sq),
                            _ => None,
                        },
                        Direction::BottomLeft | Direction::BottomRight => match pc_ex.piece.type_()
                        {
                            PieceType::K
                            | PieceType::B
                            | PieceType::S
                            | PieceType::PR
                            | PieceType::PB => Some(sq),
                            _ => None,
                        },
                        Direction::Bottom => match pc_ex.piece.type_() {
                            PieceType::K
                            | PieceType::R
                            | PieceType::G
                            | PieceType::PR
                            | PieceType::PB
                            | PieceType::PS
                            | PieceType::PN
                            | PieceType::PL
                            | PieceType::PP => Some(sq),
                            _ => None,
                        },
                        // 桂馬
                        Direction::TopRightKnight | Direction::TopLeftKnight => {
                            match pc_ex.piece.type_() {
                                PieceType::N => Some(sq),
                                _ => None,
                            }
                        }
                    };

                    if is_debug {
                        if let None = checker {
                            Beam::shoot(&format!("# get_check sq={} (End)", sq));
                        } else {
                            Beam::shoot(&format!("# get_check sq={} (Checker)", sq));
                        }
                    }
                } else {
                    // 離れたところにある長い利きの駒は ピンの頭か、チェッカーのどちらか（＾～＾）
                    let opponent = match direction {
                        // 飛、竜
                        Direction::Right | Direction::Left | Direction::Bottom => {
                            match pc_ex.piece.type_() {
                                PieceType::R | PieceType::PR => Some(sq),
                                _ => None,
                            }
                        }
                        // 角、馬
                        Direction::TopRight
                        | Direction::TopLeft
                        | Direction::BottomLeft
                        | Direction::BottomRight => match pc_ex.piece.type_() {
                            PieceType::B | PieceType::PB => Some(sq),
                            _ => None,
                        },
                        // 飛、香、竜
                        Direction::Top => match pc_ex.piece.type_() {
                            PieceType::R | PieceType::L | PieceType::PR => Some(sq),
                            _ => None,
                        },
                        // 桂馬の動きは想定してないぜ（＾～＾）
                        Direction::TopRightKnight | Direction::TopLeftKnight => None,
                    };
                    if let None = opponent {
                        if is_debug {
                            Beam::shoot(&format!("# get_check sq={} (End)", sq));
                        }
                    } else if let None = pinned {
                        if is_debug {
                            Beam::shoot(&format!("# get_check sq={} (Checker)", sq));
                        }
                        checker = opponent;
                    } else {
                        if is_debug {
                            Beam::shoot(&format!("# get_check sq={} (PinHead)", sq));
                        }
                        pin_head = opponent;
                    }
                }
                // ループ終了
                interval += 1;
                break;
            }
        } else {
            if is_debug {
                Beam::shoot(&format!("# get_check sq={}", sq));
            }
        }

        sq = Square::new((sq.number() as i8 + d_sq) as u8);
        interval += 1;
    }

    if let None = pin_head {
        // ピン頭は無かったので、合い駒もありません
        pinned = None;
        if is_debug {
            Beam::shoot("# get_check cancel pinned");
        }
    }
    if let None = checker {
        // チェッカーは無かったので、追加した分を減らします
        sq_list.truncate(sq_list.len() - interval);
        if is_debug {
            Beam::shoot(&format!("# get_check cancel interval={}", interval));
        }
    }

    (pinned, pin_head, checker)
}
