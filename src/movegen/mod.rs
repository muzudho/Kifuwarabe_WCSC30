//!
//! 現局面を使った指し手生成☆（＾～＾）
//!

use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::features::HandAddress;
use crate::entities::cosmic::smart::features::PieceType;
use crate::entities::cosmic::smart::square::{
    Angle, RelAdr, FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4, RANK_6, RANK_7,
    RANK_8, RANK_9,
};
use crate::entities::move_::new_move;
use crate::entities::spaceship::equipment::Beam;
use crate::position::destructure_move;
use crate::position::file;
use crate::position::is_board_square;
use crate::position::is_hand_square;
use crate::position::position::{PieceNum, Position};
use crate::position::rank;
use crate::position::square_from;
use crate::position::square_offset;
use crate::position::square_rotate_180;
use crate::position::square_to_hand_address;
use crate::position::square_wall;
use crate::position::to_move_code;
use crate::position::Square;
use crate::take1base::Move;
use crate::take1base::Piece;
use crate::view::print_move_list;
use crate::view::print_sq_list;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct PieceEx {
    /// 深い意味は無く Stockfish の Piece（＾～＾）
    pub piece: Piece,
    /// 将棋の駒の背番号だぜ☆（＾～＾）
    pub num: PieceNum,
}
impl PieceEx {
    pub fn new(piece: Piece, num: PieceNum) -> Self {
        PieceEx {
            piece: piece,
            num: num,
        }
    }
}
impl fmt::Debug for PieceEx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PieceEx({} {:?})", self.piece, self.num)
    }
}

#[derive(Clone, Copy)]
pub struct Mobility {
    pub angle: Angle,
    pub agility: Agility,
}
impl Mobility {
    pub fn new(angle: Angle, agility: Agility) -> Self {
        Mobility {
            angle: angle,
            agility: agility,
        }
    }
}

/// 向き
/// TODO 桂馬
/// TODO 先後
#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
}

// 筋は -1 すると右（＾～＾）
// 段は -1 すると上（＾～＾）
const DIRECTIONS_SQ: [i8; 8] = [
    -10, // 右方向
    -11, // 右上方向
    -1,  // 上方向
    9,   // 左上方向
    10,  // 左方向
    11,  // 左下方向
    1,   // 下方向
    -9,  // 右下方向
];

// 玉が移動したとき、敵の長い利きが当たっているかどうか。
// ただし、駒が動く前の position であることに注意してください。
fn king_is_adjacent_opponent_long_control(
    us: Phase,
    position_before_move: &Position,
    ksq_from: Square,
    ksq_to: Square,
    direction: Direction,
) -> bool {
    let d_sq = DIRECTIONS_SQ[direction as usize];
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
        } else if let Some(pc_ex) = position_before_move.piece_at(adjacent_sq as u8) {
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

                // TODO 桂馬
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
                };
            }
        }

        distance += 1;
    }

    false
}

// 隣接する敵の１マスの利きが利いているかどうか
fn is_adjacent_opponent_control(
    us: Phase,
    position: &Position,
    ksq_to: Square,
    direction: Direction,
) -> bool {
    let d_sq = DIRECTIONS_SQ[direction as usize];

    // 隣のマス
    let adjacent_sq = (ksq_to as i8 + d_sq) as u8;
    // Beam::shoot(&format!(
    //     "is_adjacent_opponent_control d_file={} d_rank={} adjacent_sq={}",
    //     d_file, d_rank, adjacent_sq
    // ));

    if let Some(pc_ex) = position.piece_at(adjacent_sq) {
        if us != pc_ex.piece.phase() {
            // 敵の駒なら
            // TODO 桂馬
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
                    PieceType::K | PieceType::B | PieceType::S | PieceType::PR | PieceType::PB => {
                        return true
                    }
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
            };
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
fn check_checker_pin(
    us: Phase,
    position: &Position,
    ksq: Square,
    direction: Direction,
    sq_list: &mut Vec<Square>,
) -> (Option<Square>, Option<Square>, Option<Square>) {
    let d_sq = DIRECTIONS_SQ[direction as usize];
    Beam::shoot(&format!(
        "# check_checker_pin us={} ksq={} d_sq={}",
        us, ksq, d_sq
    ));

    let mut sq = (ksq as i8 + d_sq) as u8;
    let mut pinned: Option<Square> = None; // 合い駒か、ただの自駒
    let mut pin_head: Option<Square> = None; // ピンしてる駒
    let mut checker: Option<Square> = None; // チェック駒
    let mut interval = 0;
    while FILE_1 <= file(sq) && file(sq) < FILE_10 && RANK_1 <= rank(sq) && rank(sq) < RANK_10 {
        sq_list.push(sq);

        if let Some(pc_ex) = position.piece_at(sq) {
            if us == pc_ex.piece.phase() {
                // 合い駒か、ただの自駒か
                if let None = pinned {
                    // とりあえず 合い駒 候補
                    pinned = Some(sq);
                    Beam::shoot(&format!("# check_checker_pin sq={} (Pinned?)", sq));
                } else {
                    // 味方の駒が２枚あれば長い利きは当たっていません
                    // ループ終了
                    Beam::shoot(&format!("# check_checker_pin sq={} (End)", sq));
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
                    };

                    if let None = checker {
                        Beam::shoot(&format!("# check_checker_pin sq={} (End)", sq));
                    } else {
                        Beam::shoot(&format!("# check_checker_pin sq={} (Checker)", sq));
                    }
                } else {
                    // 離れたところにある長い利きの駒は ピンの頭か、チェッカーのどちらか（＾～＾）
                    let opponent = match direction {
                        Direction::Right | Direction::Left | Direction::Bottom => {
                            match pc_ex.piece.type_() {
                                PieceType::R | PieceType::PR => Some(sq),
                                _ => None,
                            }
                        }
                        Direction::TopRight
                        | Direction::TopLeft
                        | Direction::BottomLeft
                        | Direction::BottomRight => match pc_ex.piece.type_() {
                            PieceType::B | PieceType::PB => Some(sq),
                            _ => None,
                        },
                        Direction::Top => match pc_ex.piece.type_() {
                            PieceType::R | PieceType::L | PieceType::PR => Some(sq),
                            _ => None,
                        },
                    };
                    if let None = opponent {
                        Beam::shoot(&format!("# check_checker_pin sq={} (End)", sq));
                    } else if let None = pinned {
                        Beam::shoot(&format!("# check_checker_pin sq={} (Checker)", sq));
                        checker = opponent;
                    } else {
                        Beam::shoot(&format!("# check_checker_pin sq={} (PinHead)", sq));
                        pin_head = opponent;
                    }
                }
                // ループ終了
                interval += 1;
                break;
            }
        } else {
            Beam::shoot(&format!("# check_checker_pin sq={}", sq));
        }

        sq = (sq as i8 + d_sq) as u8;
        interval += 1;
    }

    if let None = pin_head {
        // ピン頭は無かったので、合い駒もありません
        pinned = None;
        Beam::shoot("# check_checker_pin cancel pinned");
    }
    if let None = checker {
        // チェッカーは無かったので、追加した分を減らします
        sq_list.truncate(sq_list.len() - interval);
        Beam::shoot(&format!("# check_checker_pin cancel interval={}", interval));
    }

    (pinned, pin_head, checker)
}

/// 指し手生成区分（＾～＾）
pub enum GenType {
    // 王手されてるから回避しろよ（＾～＾）
    Evasion,
    // 王手されてないから普通にしろよ（＾～＾）
    NonEvasion,
}

/// Pseudo legal move(疑似合法手)☆（＾～＾）
///
/// 先手の連続王手の千日手とか、空き王手とか、駒を見ただけでは調べられないだろ☆（＾～＾）
/// 棋譜や盤面を見ず、駒だけで調べる合法手が Pseudo legal move だぜ☆（＾～＾）
///
/// 二歩とか、打った後で調べた方が高速になるはずだが、探索部がまだできてないので、指し手生成の中でチェックしているぜ☆（＾～＾）
/// 香を２段目に打たないとか強い将棋を目指すことは　まだやってないぜ☆（＾～＾）
pub struct PseudoLegalMoves {}
impl PseudoLegalMoves {
    ///
    /// 現局面の、任意の移動先升の、
    /// - 盤上の駒の移動
    /// - 打
    /// の指し手を生成。
    ///
    /// 王手回避漏れや、千日手などのチェックは行っていない
    ///
    /// https://doc.rust-lang.org/std/ops/trait.FnMut.html
    ///
    /// Arguments
    /// ---------
    /// * `us` - どちらの手番か☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    ///
    /// # Returns
    ///
    /// 指し手の一覧
    pub fn generate(us: Phase, position: &Position) -> Vec<Move> {
        // TODO その手を指して、王手が解消されない手は除外したい
        // 本書では、「離れた王手」は玉とチェッカーの間に１マス以上の空きマスがあるものとします。また、桂を含みません。
        //
        // 離れた王手回避
        // -------------
        // 1. 離れた王手が２つなら、玉を動かすしかない
        // 2. 離れた王手が１つなら、そのチェッカーのあるマスから玉の手前までのマスへ、玉以外の味方の駒を動かす（打含む）
        // （離れた利きのチェックでは、玉でチェッカーを取り返すことはできない）

        // TODO 自玉の位置検索
        let ksq = match us {
            Phase::First => position.location_at(PieceNum::King1),
            Phase::Second => position.location_at(PieceNum::King2),
        };

        // Beam::shoot(&format!("# generate ksq={}", ksq));

        if !is_board_square(ksq) {
            panic!("(Err.93) ksq fail")
        }

        // 合い駒(Pinned)検索
        // ピンの頭検索。ただちに回避する必要はない
        // スライディング・チェッカー(Sliding Checker)検索
        let mut pinned_list = Vec::<Square>::new();
        let mut pin_head_list = Vec::<Square>::new();
        let mut checker_list = Vec::<Square>::new();
        let mut long_control_sq_list = Vec::<Square>::new();

        // とりあえず 合い駒(Pinned) は今のところ 動かさないことにするぜ（＾～＾）
        let directions = [
            Direction::Right,
            Direction::TopRight,
            Direction::Top,
            Direction::TopLeft,
            Direction::Left,
            Direction::BottomLeft,
            Direction::Bottom,
            Direction::BottomRight,
        ];
        for direction in directions {
            let (pinned, pin_head, checker) =
                check_checker_pin(us, position, ksq, direction, &mut long_control_sq_list);
            if let Some(pinned) = pinned {
                pinned_list.push(pinned);
            }
            if let Some(pin_head) = pin_head {
                pin_head_list.push(pin_head);
            }
            if let Some(checker) = checker {
                checker_list.push(checker);
            }
        }
        print_sq_list("pinned_list", &pinned_list);
        print_sq_list("pin_head_list", &pin_head_list);
        print_sq_list("checker_list", &checker_list);
        print_sq_list("long_control_sq_list", &long_control_sq_list);

        let gen_type = if checker_list.is_empty() {
            // TODO チェッカーがいなかったら、非回避(Non-evasions)モードへ
            GenType::NonEvasion
        } else {
            // TODO チェッカーがいたら、ただちに 王手回避(Evasions)モードへ
            GenType::Evasion
        };

        let mut move_list = Vec::<Move>::new();

        if 2 <= checker_list.len() {
            // チェッカーが２つあったら、玉が移動するしかない
            PseudoLegalMoves::generate_king(us, position, &mut move_list);
        } else {
            // チェッカーが１つ以下なら
            PseudoLegalMoves::generate_non_evasion(us, position, &mut move_list);
            print_move_list("generate_non_evasion", position, &move_list);
            // とりあえず、合い駒を動かす手を除外します
            // TODO 合い駒でも、動かしていい方向はあるはず
            move_list.retain(|particle| {
                let (from, _, _) = destructure_move(*particle);
                let retain = !pinned_list.contains(&from);
                if !retain {
                    Beam::shoot(&format!("# remove pinned-move={}", to_move_code(*particle)));
                }
                retain
            });

            match gen_type {
                GenType::Evasion => {
                    // 合い駒になるような動き以外の、自玉以外の味方の動きを除外
                    // 残す駒だけ 真を返してください
                    move_list.retain(|particle| {
                        let (from, to, _) = destructure_move(*particle);
                        if from == ksq {
                            Beam::shoot(&format!("# retain king={}", to_move_code(*particle)));
                            true
                        } else {
                            // 利きを止めるような動きでなければ除外
                            let retain = long_control_sq_list.contains(&to);
                            if !retain {
                                Beam::shoot(&format!(
                                    "# remove not-pinned-move={}",
                                    to_move_code(*particle)
                                ));
                            }
                            retain
                        }
                    });
                }
                _ => {}
            }
        }

        // TODO 玉の自殺手を除外したい（＾～＾）
        move_list.retain(|particle| {
            let (from, to, _) = destructure_move(*particle);
            if from == ksq {
                // Control 1～12
                let c1 = is_adjacent_opponent_control(us, position, to, Direction::Right);
                let c2 = is_adjacent_opponent_control(us, position, to, Direction::TopRight);
                let c3 = is_adjacent_opponent_control(us, position, to, Direction::Top);
                let c4 = is_adjacent_opponent_control(us, position, to, Direction::TopLeft);
                let c5 = is_adjacent_opponent_control(us, position, to, Direction::Left);
                let c6 = is_adjacent_opponent_control(us, position, to, Direction::BottomLeft);
                let c7 = is_adjacent_opponent_control(us, position, to, Direction::Bottom);
                let c8 = is_adjacent_opponent_control(us, position, to, Direction::BottomRight);
                let c9 = king_is_adjacent_opponent_long_control(us, position, from,to, Direction::TopRight);
                let c10 = king_is_adjacent_opponent_long_control(us, position,from, to, Direction::TopLeft);
                let c11 =
                king_is_adjacent_opponent_long_control(us, position, from,to, Direction::BottomLeft);
                let c12 =
                king_is_adjacent_opponent_long_control(us, position, from,to, Direction::BottomRight);
                let control =
                    c1 || c2 || c3 || c4 || c5 || c6 || c7 || c8 || c9 || c10 || c11 || c12;

                // Beam::shoot(&format!(
                //     "# suicide-check from={} to={} control={}",
                //     from, to, control
                // ));
                if control {
                    Beam::shoot(&format!(
                        "# remove suicide-move={} from={} to={} control={} c1={} c2={} c3={} c4={} c5={} c6={} c7={} c8={} c9={} c10={} c11={} c12={}",
                        to_move_code(*particle),
                        from,to,control,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12
                    ));
                }
                !control
            } else {
                // 玉以外の駒の動きは残す
                true
            }
        });

        move_list
    }

    /// 盤上の玉の指し手だけ生成（＾～＾）
    fn generate_king(us: Phase, position: &Position, move_list: &mut Vec<Move>) {
        let ksq = match us {
            Phase::First => position.location_at(PieceNum::King1),
            Phase::Second => position.location_at(PieceNum::King2),
        };
        // 盤上の駒☆（＾～＾）
        let pc_ex = if let Some(pc_ex) = position.piece_at(ksq) {
            pc_ex
        } else {
            panic!("ksq fail {:?}", ksq)
        };
        // 座標ではなく、駒の背番号で検索
        PseudoLegalMoves::start_on_board(us, ksq, &pc_ex, position, move_list)
    }

    fn generate_non_evasion(us: Phase, position: &Position, move_list: &mut Vec<Move>) {
        // 座標ではなく、駒の背番号で検索
        position.for_some_pieces_on_list40(us, &mut |sq, pc_ex| {
            if is_board_square(sq) {
                PseudoLegalMoves::start_on_board(us, sq, &pc_ex, position, move_list)
            } else if is_hand_square(sq) {
                PseudoLegalMoves::make_drop(us, square_to_hand_address(sq), position, move_list);
            } else {
                std::panic::panic_any(Beam::trouble(
                    "(Err.94) なんで駒が作業中なんだぜ☆（＾～＾）！",
                ))
            }
        });
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `pc_ex` - 駒だぜ☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `move_list` - 指し手一覧☆（＾～＾）
    ///
    /// Returns
    /// -------
    /// F1:
    /// * 指し手ハッシュ
    /// * 移動先にあった駒
    fn start_on_board(
        us: Phase,
        from: Square,
        pc_ex: &PieceEx,
        position: &Position,
        move_list: &mut Vec<Move>,
    ) {
        let moving = &mut |to, promotability, _agility, move_permission: Option<MovePermission>| {
            let pseudo_captured = position.piece_at(to);

            let (ok, space) = if let Some(pseudo_captured_val) = pseudo_captured {
                if pseudo_captured_val.piece.phase() == us {
                    // 味方の駒を取った☆（＾～＾）なしだぜ☆（＾～＾）！
                    (false, false)
                } else {
                    (true, false)
                }
            } else {
                (true, true)
            };

            if ok {
                // 成れるかどうかの判定☆（＾ｑ＾）
                use crate::movegen::Promotability::*;
                let promotion = match &promotability {
                    Forced => true,
                    _ => false,
                };

                // 成りじゃない場合は、行き先のない動きを制限されるぜ☆（＾～＾）
                let forbidden = if let Some(move_permission_val) = move_permission {
                    if move_permission_val.check(to) {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                };

                match &promotability {
                    Any => {
                        // 成ったり、成れなかったりできるとき。
                        if !forbidden {
                            let m = new_move(us, Some(from), to, false, None);
                            move_list.push(m);
                        }
                        let m = new_move(us, Some(from), to, true, None);
                        move_list.push(m);
                    }
                    _ => {
                        // 成れるか、成れないかのどちらかのとき。
                        if promotion || !forbidden {
                            let m = new_move(us, Some(from), to, promotion, None);
                            move_list.push(m);
                        }
                    }
                };
            }

            !space
        };

        Area::piece_of(pc_ex.piece.type_(), us, from, moving);
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `move_list` - 指し手一覧☆（＾～＾）
    fn make_drop(us: Phase, adr: HandAddress, position: &Position, move_list: &mut Vec<Move>) {
        if let Some(pc_ex) = position.last_hand(adr) {
            // 打つぜ☆（＾～＾）
            let drop = &mut |to| {
                if let None = position.piece_at(to) {
                    // 駒が無いところに打つ
                    use crate::take1base::Piece::*;
                    match pc_ex.piece {
                        P1 | P2 => {
                            // ひよこ　は２歩できない☆（＾～＾）
                            if position.exists_pawn_on_file(us, file(to)) {
                                return;
                            }
                        }
                        _ => {}
                    }
                    let m = new_move(
                        us,
                        None,                                     // 駒台
                        to,                                       // どの升へ行きたいか
                        false,                                    // 打に成りは無し
                        Some(pc_ex.piece.hand_address().type_()), // 打った駒種類
                    );
                    move_list.push(m);
                }
            };

            // 駒を持っていれば
            let ty = adr.type_();
            use crate::entities::cosmic::smart::features::HandAddressType::*;
            match ty {
                // 歩、香
                Pawn | Lance => Area::drop_pawn_lance(us, drop),
                // 桂
                Knight => Area::drop_knight(us, drop),
                // それ以外の駒が打てる範囲は盤面全体。
                _ => Area::for_all(drop),
            }
        }
    }
}

/// 次の升☆（＾～＾）
pub struct Area {}
impl Area {
    /// 全升の面積だぜ☆（＾～＾）駒を打つときに使うぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn for_all<F1>(callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank in RANK_1..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                callback(square_from(file, rank));
            }
        }
    }

    /// 先手から見た盤上の駒の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `piece_type` - 駒の種類だぜ☆（＾～＾）
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `hopping` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    /// * `sliding` -
    fn piece_of<F1>(piece_type: PieceType, us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        match piece_type {
            PieceType::P => Area::pawn(us, from, moving),
            PieceType::L => Area::lance(us, from, moving),
            PieceType::N => Area::knight(us, from, moving),
            PieceType::S => Area::silver(us, from, moving),
            PieceType::G => Area::gold(us, from, moving),
            PieceType::K => Area::king(from, moving),
            PieceType::B => Area::bishop(us, from, moving),
            PieceType::R => Area::rook(us, from, moving),
            PieceType::PP => Area::gold(us, from, moving),
            PieceType::PL => Area::gold(us, from, moving),
            PieceType::PN => Area::gold(us, from, moving),
            PieceType::PS => Area::gold(us, from, moving),
            PieceType::PB => Area::horse(from, moving),
            PieceType::PR => Area::dragon(from, moving),
        }
    }

    /// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn pawn<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| {
            Promoting::pawn_lance(us, to, moving, Some(MovePermission::from_pawn_or_lance(us)))
        };

        for mobility in PieceType::P.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn lance<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| {
            Promoting::pawn_lance(us, to, moving, Some(MovePermission::from_pawn_or_lance(us)))
        };

        for mobility in PieceType::L.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn knight<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| {
            Promoting::knight(us, to, moving, Some(MovePermission::from_knight(us)))
        };

        for mobility in PieceType::N.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn silver<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| Promoting::silver(us, from, to, moving);

        for mobility in PieceType::S.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn gold<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| moving(to, Promotability::Deny, Agility::Hopping, None);

        for mobility in PieceType::G.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 盤上の玉の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn king<F1>(from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| moving(to, Promotability::Deny, Agility::Hopping, None);

        for mobility in PieceType::K.mobility().iter() {
            Area::move_(&None, from, *mobility, moving);
        }
    }

    /// 盤上の角の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn bishop<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| Promoting::bishop_rook(us, from, to, moving);
        for mobility in PieceType::B.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 盤上の飛の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn rook<F1>(us: Phase, from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, _agility| Promoting::bishop_rook(us, from, to, moving);
        for mobility in PieceType::R.mobility().iter() {
            Area::move_(&Some(us), from, *mobility, moving);
        }
    }

    /// 盤上の馬の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn horse<F1>(from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        let moving = &mut |to, agility| moving(to, Promotability::Deny, agility, None);

        for mobility in PieceType::PB.mobility().iter() {
            Area::move_(&None, from, *mobility, moving);
        }
    }

    /// 盤上の竜の動けるマスだぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `moving` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
    fn dragon<F1>(from: Square, moving: &mut F1)
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        {
            let moving = &mut |to, agility| moving(to, Promotability::Deny, agility, None);

            for mobility in PieceType::PR.mobility().iter() {
                Area::move_(&None, from, *mobility, moving);
            }
        }
    }

    /// 先手から見た歩、香車の打てる面積だぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn drop_pawn_lance<F1>(us: Phase, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        // 180°回転とかするより、for文の方を変えた方が高速だろ……☆（＾～＾）
        let (min_rank, max_rank) = if us == Phase::First {
            (RANK_2, RANK_10)
        } else {
            (RANK_1, RANK_9)
        };

        for rank in min_rank..max_rank {
            for file in (FILE_1..FILE_10).rev() {
                callback(square_from(file, rank));
            }
        }
    }

    /// 先手から見た桂馬の打てる面積だぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` - 手番☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    pub fn drop_knight<F1>(us: Phase, callback: &mut F1)
    where
        F1: FnMut(Square),
    {
        for rank in RANK_3..RANK_10 {
            for file in (FILE_1..FILE_10).rev() {
                let mut sq = square_from(file, rank);
                if us == Phase::Second {
                    sq = square_rotate_180(sq);
                }

                callback(sq);
            }
        }
    }

    /// 盤上の駒を指すぜ☆（＾～＾）
    ///
    /// Arguments
    /// ---------
    /// * `us` - 先手か後手か、関係ないか☆（＾～＾）先後同型なら関係ないしな☆（＾～＾）
    /// * `start` - 移動元升☆（＾～＾）
    /// * `angle` - 角度☆（＾～＾）
    /// * `agility` - 動き方☆（＾～＾）
    /// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
    fn move_<F1>(us: &Option<Phase>, start: Square, mobility: Mobility, moving: &mut F1)
    where
        F1: FnMut(Square, Agility) -> bool,
    {
        let angle = if let Some(friend_val) = us {
            // 先後同型でない駒は、後手なら１８０°回転だぜ☆（＾～＾）
            if *friend_val == Phase::Second {
                mobility.angle.rotate180()
            } else {
                mobility.angle
            }
        } else {
            // 先後同型だからそのままだぜ☆（＾～＾）
            mobility.angle
        };

        match mobility.agility {
            Agility::Sliding => {
                let mut cur = start;
                let r = RelAdr::new(1, 0).rotate(mobility.angle).clone();

                loop {
                    // 西隣から反時計回りだぜ☆（＾～＾）
                    cur = square_offset(cur, &r);
                    if square_wall(cur) {
                        break;
                    }

                    if moving(cur, mobility.agility) {
                        break;
                    }
                }
            }
            // 桂馬専用☆（＾～＾）行き先の無いところに置いてないはずだぜ☆（＾～＾）
            Agility::Knight => {
                let mut cur = start;

                // 西隣から反時計回りだぜ☆（＾～＾）
                cur = square_offset(cur, &angle.west_ccw_double_rank());
                if !square_wall(cur) {
                    moving(cur, mobility.agility);
                }
            }
            Agility::Hopping => {
                let mut cur = start;

                // 西隣から反時計回りだぜ☆（＾～＾）
                cur = square_offset(cur, &angle.west_ccw());
                if !square_wall(cur) {
                    moving(cur, mobility.agility);
                }
            }
        }
    }
}

/// 機敏性。
#[derive(Clone, Copy, Debug)]
pub enum Agility {
    /// 隣へ１つ進む駒。
    Hopping,
    /// 長い利き。
    Sliding,
    /// 桂馬。
    Knight,
}

enum Promotability {
    /// 成ることはできないぜ☆（＾～＾）
    Deny,
    /// 成る、成らない両方あるぜ☆（＾～＾）
    Any,
    /// 必ず成れだぜ☆（＾～＾）
    Forced,
}

/// 行き先があるかないかのチェックに使うぜ☆（＾～＾）
/// 成れるときは使わないぜ☆（＾～＾）
struct MovePermission {
    min_rank: u8,
    max_rank: u8,
}
impl MovePermission {
    fn from_pawn_or_lance(us: Phase) -> Self {
        // ▲P,▲L　は１段目(▽P,▽L　は９段目)には進めない
        match us {
            Phase::First => MovePermission {
                min_rank: 2,
                max_rank: 9,
            },
            Phase::Second => MovePermission {
                min_rank: 1,
                max_rank: 8,
            },
        }
    }
    fn from_knight(us: Phase) -> Self {
        // ▲N　は１、２段目(▽N　は８、９段目)には進めない
        match us {
            Phase::First => MovePermission {
                min_rank: 3,
                max_rank: 9,
            },
            Phase::Second => MovePermission {
                min_rank: 1,
                max_rank: 7,
            },
        }
    }
    fn check(&self, to: Square) -> bool {
        if rank(to) < self.min_rank || self.max_rank < rank(to) {
            return false;
        }
        true
    }
}
impl fmt::Debug for MovePermission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rank{}~{})", self.min_rank, self.max_rank)
    }
}

/// 成れるか、成れないか☆（＾～＾）
struct Promoting {}
impl Promoting {
    /// 歩と香のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    /// * `callback` -
    /// * `move_permission` - 成らずに一番奥の段に移動することはできません。
    fn pawn_lance<F1>(
        us: Phase,
        to: Square,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(us, to) {
            // 自陣から見て一番奥の段
            callback(to, Promotability::Forced, Agility::Hopping, move_permission)
        } else if Promoting::is_second_third_farthest_rank_from_friend(us, to) {
            // 自陣から見て二番、三番目の奥の段
            callback(to, Promotability::Any, Agility::Hopping, move_permission)
        } else {
            callback(to, Promotability::Deny, Agility::Hopping, move_permission)
        }
    }

    /// 桂のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    /// * `callback` -
    /// * `move_permission` - 成らずに奥から２番目の段に移動することはできません。
    fn knight<F1>(
        us: Phase,
        to: Square,
        callback: &mut F1,
        move_permission: Option<MovePermission>,
    ) -> bool
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_first_second_farthest_rank_from_friend(us, to) {
            callback(to, Promotability::Forced, Agility::Knight, move_permission)
        } else if Promoting::is_third_farthest_rank_from_friend(us, to) {
            callback(to, Promotability::Any, Agility::Knight, move_permission)
        } else {
            callback(to, Promotability::Deny, Agility::Knight, move_permission)
        }
    }

    /// 銀のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 自陣から見て奥から１～３段目に入るときに成れます。元位置が３段目のときは、動けば成るか選べます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `from` -
    /// * `to` -
    /// * `callback` -
    fn silver<F1>(us: Phase, from: Square, to: Square, callback: &mut F1) -> bool
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(us, from) {
            callback(to, Promotability::Any, Agility::Hopping, None)
        } else if Promoting::is_opponent_region(us, to) {
            callback(to, Promotability::Any, Agility::Hopping, None)
        } else {
            callback(to, Promotability::Deny, Agility::Hopping, None)
        }
    }

    /// 角と飛のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `from` -
    /// * `to` -
    /// * `callback` -
    fn bishop_rook<F1>(us: Phase, from: Square, to: Square, callback: &mut F1) -> bool
    where
        F1: FnMut(Square, Promotability, Agility, Option<MovePermission>) -> bool,
    {
        if Promoting::is_opponent_region(us, from) || Promoting::is_opponent_region(us, to) {
            callback(to, Promotability::Any, Agility::Sliding, None)
        } else {
            callback(to, Promotability::Deny, Agility::Sliding, None)
        }
    }

    /// 自陣から見て、一番遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    fn is_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && rank(to) < RANK_2) || (us == Phase::Second && RANK_8 < rank(to))
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    fn is_first_second_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && rank(to) < RANK_3) || (us == Phase::Second && RANK_7 < rank(to))
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    fn is_second_third_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && RANK_1 < rank(to) && rank(to) < RANK_4)
            || (us == Phase::Second && RANK_6 < rank(to) && rank(to) < RANK_9)
    }
    /// 自陣から見て、三番目に遠いの段
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    fn is_third_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && rank(to) == RANK_3) || (us == Phase::Second && RANK_7 == rank(to))
    }
    /// 敵陣
    ///
    /// Arguments
    /// ---------
    ///
    /// * `us` -
    /// * `to` -
    fn is_opponent_region(us: Phase, to: Square) -> bool {
        (us == Phase::First && rank(to) < RANK_4) || (us == Phase::Second && RANK_6 < rank(to))
    }
}
