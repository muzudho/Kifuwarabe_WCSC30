//!
//! 現局面を使った指し手生成☆（＾～＾）
//!
mod check;

use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::smart::features::HandPiece;
use crate::entities::cosmic::smart::features::PieceType;
use crate::entities::move_::new_move;
use crate::entities::spaceship::equipment::Beam;
use crate::position::destructure_move;
use crate::position::position::{PieceNum, Position};
use crate::position::rotation::Angle;
use crate::position::square_to_hand_piece;
use crate::position::to_move_code;
use crate::position::RelAdr;
use crate::position::Square;
use crate::position::{
    FILE_1, FILE_10, RANK_1, RANK_10, RANK_2, RANK_3, RANK_4, RANK_6, RANK_7, RANK_8, RANK_9,
};
use crate::take1base::Move;
use crate::take1base::Piece;
use crate::view::print_move_list;
use crate::view::print_sq_list;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

/// 王手に関する関数の集まり
pub struct Check();

#[derive(Clone, Copy, PartialEq)]
pub struct PieceEx {
    /// Stockfish の Piece相当（＾～＾）
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
    pub move_range: MoveRange,
}
impl Mobility {
    pub fn new(angle: Angle, move_range: MoveRange) -> Self {
        Mobility {
            angle: angle,
            move_range: move_range,
        }
    }
}

/// 手番から見た向き
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
    TopRightKnight, // 先手桂右
    TopLeftKnight,  // 先手桂左
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
    pub fn generate(us: Phase, position: &Position, is_debug: bool) -> Vec<Move> {
        // その手を指して、王手が解消されない手は除外したい
        // 本書では、「離れた王手」は玉とチェッカーの間に１マス以上の空きマスがあるものとします。また、桂を含みません。
        //
        // 離れた王手回避
        // -------------
        // 1. 離れた王手が２つなら、玉を動かすしかない
        // 2. 離れた王手が１つなら、そのチェッカーのあるマスから玉の手前までのマスへ、玉以外の味方の駒を動かす（打含む）
        // （離れた利きのチェックでは、玉でチェッカーを取り返すことはできない）

        // 自玉の位置検索
        let ksq = match us {
            Phase::First => position.location_at(PieceNum::King1),
            Phase::Second => position.location_at(PieceNum::King2),
        };

        if is_debug {
            Beam::shoot(&format!("# generate ksq={}", ksq));
        }

        if !ksq.is_board() {
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
            Direction::TopRightKnight,
            Direction::TopLeftKnight,
        ];
        for direction in directions {
            let (pinned, pin_head, checker) = Check::get_square(
                us,
                position,
                ksq,
                direction,
                &mut long_control_sq_list,
                is_debug,
            );
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

        if is_debug {
            print_sq_list("pinned_list", &pinned_list);
            print_sq_list("pin_head_list", &pin_head_list);
            print_sq_list("checker_list", &checker_list);
            print_sq_list("long_control_sq_list", &long_control_sq_list);
        }

        let gen_type = if checker_list.is_empty() {
            // チェッカーがいなかったら、非回避(Non-evasions)モードへ
            GenType::NonEvasion
        } else {
            // チェッカーがいたら、ただちに 王手回避(Evasions)モードへ
            GenType::Evasion
        };

        let mut move_list = Vec::<Move>::new();

        if 2 <= checker_list.len() {
            // チェッカーが２つあったら、玉が移動するしかない
            PseudoLegalMoves::generate_king(us, position, &mut move_list);
        } else {
            // チェッカーが１つ以下なら
            PseudoLegalMoves::generate_non_evasion(us, position, &mut move_list);
            if is_debug {
                print_move_list("generate_non_evasion", position, &move_list);
            }
            // とりあえず、合い駒を動かす手を除外します
            // TODO 合い駒でも、動かしていい方向はあるはず
            move_list.retain(|particle| {
                let (from, _, _) = destructure_move(*particle);
                let retain = !pinned_list.contains(&from);
                if is_debug && !retain {
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
                            if is_debug {
                                Beam::shoot(&format!("# retain king={}", to_move_code(*particle)));
                            }
                            true
                        } else {
                            // 利きを止めるような動きでなければ除外
                            let retain = long_control_sq_list.contains(&to);
                            if is_debug && !retain {
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

        // 自玉が移動して、相手の利きに飛び込む手（自殺手）を除外したい（＾～＾）
        move_list.retain(|particle| {
            let (from, to, _) = destructure_move(*particle);
            if from == ksq {
                // Control 1～12
                let r = Check::is_adjacent(us, position, to, Direction::Right);
                let tr = Check::is_adjacent(us, position, to, Direction::TopRight);
                let t = Check::is_adjacent(us, position, to, Direction::Top);
                let tl = Check::is_adjacent(us, position, to, Direction::TopLeft);
                let l = Check::is_adjacent(us, position, to, Direction::Left);
                let bl = Check::is_adjacent(us, position, to, Direction::BottomLeft);
                let b = Check::is_adjacent(us, position, to, Direction::Bottom);
                let br = Check::is_adjacent(us, position, to, Direction::BottomRight);
                // 桂馬の利き
                let trn = Check::is_adjacent(us, position, to, Direction::TopRightKnight);
                let tln = Check::is_adjacent(us, position, to, Direction::TopLeftKnight);
                // 飛、香、竜の動き
                let long_r = Check::is_long(
                    us,
                    position,
                    from,
                    to,
                    Direction::Right,
                );
                let long_t =
                Check::is_long(us, position, from, to, Direction::Top);
                let long_l =
                Check::is_long(us, position, from, to, Direction::Left);
                let long_b = Check::is_long(
                    us,
                    position,
                    from,
                    to,
                    Direction::Bottom,
                );
                // 角、馬の動き
                let long_tr = Check::is_long(
                    us,
                    position,
                    from,
                    to,
                    Direction::TopRight,
                );
                let long_tl = Check::is_long(
                    us,
                    position,
                    from,
                    to,
                    Direction::TopLeft,
                );
                let long_bl = Check::is_long(
                    us,
                    position,
                    from,
                    to,
                    Direction::BottomLeft,
                );
                let long_br = Check::is_long(
                    us,
                    position,
                    from,
                    to,
                    Direction::BottomRight,
                );
                let control = r
                    || tr
                    || t
                    || tl
                    || l
                    || bl
                    || b
                    || br
                    || trn
                    || tln
                    || long_r
                    || long_t
                    || long_l
                    || long_b
                    || long_tr
                    || long_tl
                    || long_bl
                    || long_br;

                if is_debug && control {
                    Beam::shoot(&format!(
                        "# remove suicide-move={:5} from={:3} to={:3} control={:5} r={:5} tr={:5} t={:5} tl={:5} l={:5} bl={:5} b={:5} br={:5} trn={:5} tln={:5} long_r={:5} long_t={:5} long_l={:5} long_b={:5} long_tr={:5} long_tl={:5} long_bl={:5} long_br={:5}",
                        to_move_code(*particle),
                        from,to,control,r,tr,t,tl,l,bl,b,br, trn, tln,long_r,long_t,long_l,long_b,long_tr,long_tl,long_bl,long_br
                    ));
                }
                !control
            } else {
                // 玉以外の駒の動きは残す
                true
            }
        });

        // TODO 指し手のオーダリングをしたいが、難しいのでシャッフルしたろ（＾～＾）
        move_list.shuffle(&mut thread_rng());

        // 指し手のオーダリングをしたいぜ☆（＾～＾） 取った駒は指し手生成の段階で調べているし☆（＾～＾）
        let mut cap = 0;
        if 1 < move_list.len() {
            for i in 0..move_list.len() {
                let (_, to, _) = destructure_move(move_list[i]);
                if let Some(_captured) = position.piece_at_board(to) {
                    // 駒を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                    // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                    move_list.swap(cap, i);
                    cap += 1;
                }
            }
            // 次は駒を取ったグループの中で、玉を取った手をグループの先頭に集めるぜ☆（＾～＾）
            let mut king = 0;
            for i in 0..cap {
                let (_, to, _) = destructure_move(move_list[i]);
                if let Some(captured) = position.piece_at_board(to) {
                    match captured.piece.type_() {
                        PieceType::K => {
                            // 玉を取った手は、リストの先頭に集めるぜ☆（＾～＾）
                            // TODO .clone()いやなんで、インデックスだけソートした方がいいのか☆（＾～＾）？
                            move_list.swap(king, i);
                            king += 1;
                        }
                        _ => {}
                    }
                } else {
                    panic!("captured fail")
                }
            }
        }
        move_list
    }

    /// 盤上の玉の指し手だけ生成（＾～＾）
    fn generate_king(us: Phase, position: &Position, move_list: &mut Vec<Move>) {
        let ksq = match us {
            Phase::First => position.location_at(PieceNum::King1),
            Phase::Second => position.location_at(PieceNum::King2),
        };
        // 盤上の駒☆（＾～＾）
        let pc_ex = if let Some(pc_ex) = position.piece_at_board(ksq) {
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
            if sq.is_board() {
                PseudoLegalMoves::start_on_board(us, sq, &pc_ex, position, move_list)
            } else if sq.is_hand() {
                PseudoLegalMoves::make_drop(us, square_to_hand_piece(sq), position, move_list);
            } else {
                std::panic::panic_any(Beam::trouble(
                    "(Err.94) なんで駒が作業中なんだぜ☆（＾～＾）！",
                ))
            }
        });
    }

    /// 盤上を見ようぜ☆（＾～＾） 盤上の駒の動きを作るぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `from` - 移動元升だぜ☆（＾～＾）
    /// * `pc_ex` - 駒だぜ☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `move_list` - 指し手一覧☆（＾～＾）
    ///
    /// # Returns
    ///
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
        let fn_make_move_list =
            &mut |to, promotability, _move_range, move_permission: Option<MovePermission>| {
                let pseudo_captured = position.piece_at_board(to);

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
                                let m = new_move(from, to, false);
                                move_list.push(m);
                            }
                            let m = new_move(from, to, true);
                            move_list.push(m);
                        }
                        _ => {
                            // 成れるか、成れないかのどちらかのとき。
                            if promotion || !forbidden {
                                let m = new_move(from, to, promotion);
                                move_list.push(m);
                            }
                        }
                    };
                }

                !space
            };

        // 先手から見た盤上の駒の動けるマスだぜ☆（＾～＾）
        //
        // # Arguments
        //
        // * `piece_type` - 駒の種類だぜ☆（＾～＾）
        // * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
        // * `from` - 移動元升だぜ☆（＾～＾）
        // * `hopping` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
        // * `sliding` -
        match pc_ex.piece.type_() {
            PieceType::P => gen_pawn(us, from, fn_make_move_list),
            PieceType::L => gen_lance(us, from, fn_make_move_list),
            PieceType::N => gen_knight(us, from, fn_make_move_list),
            PieceType::S => gen_silver(us, from, fn_make_move_list),
            PieceType::G => gen_gold(us, from, fn_make_move_list),
            PieceType::K => gen_king(from, fn_make_move_list),
            PieceType::B => gen_bishop(us, from, fn_make_move_list),
            PieceType::R => gen_rook(us, from, fn_make_move_list),
            PieceType::PP => gen_gold(us, from, fn_make_move_list),
            PieceType::PL => gen_gold(us, from, fn_make_move_list),
            PieceType::PN => gen_gold(us, from, fn_make_move_list),
            PieceType::PS => gen_gold(us, from, fn_make_move_list),
            PieceType::PB => gen_horse(from, fn_make_move_list),
            PieceType::PR => gen_dragon(from, fn_make_move_list),
        }
    }

    /// 駒台を見ようぜ☆（＾～＾） 駒台の駒の動きを作るぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
    /// * `position` - 現局面の盤上だぜ☆（＾～＾）
    /// * `move_list` - 指し手一覧☆（＾～＾）
    fn make_drop(us: Phase, adr: HandPiece, position: &Position, move_list: &mut Vec<Move>) {
        if let Some(pc_ex) = position.last_hand(adr) {
            // 打つぜ☆（＾～＾）
            let drop = &mut |to| {
                if let None = position.piece_at_board(to) {
                    // 駒が無いところに打つ
                    use crate::take1base::Piece::*;
                    match pc_ex.piece {
                        P1 | P2 => {
                            // ひよこ　は２歩できない☆（＾～＾）
                            if position.exists_pawn_on_file(us, to.file()) {
                                return;
                            }
                        }
                        _ => {}
                    }
                    let m = new_move(
                        pc_ex.piece.hand_piece().square(), // 駒台
                        to,                                // どの升へ行きたいか
                        false,                             // 打に成りは無し
                    );
                    move_list.push(m);
                }
            };

            // 駒を持っていれば
            let ty = adr.type_();
            use crate::entities::cosmic::smart::features::HandType::*;
            match ty {
                // 歩、香
                Pawn | Lance => drop_pawn_lance(us, drop),
                // 桂
                Knight => drop_knight(us, drop),
                // それ以外の駒が打てる範囲は盤面全体。
                _ => foreach_square_in_board(drop),
            }
        }
    }
}

/// 全升の面積だぜ☆（＾～＾）駒を打つときに使うぜ☆（＾～＾）
///
/// # Arguments
///
/// * `callback` - 絶対番地を受け取れだぜ☆（＾～＾）
pub fn foreach_square_in_board<F1>(fn_make_move_list: &mut F1)
where
    F1: FnMut(Square),
{
    for rank in RANK_1..RANK_10 {
        for file in (FILE_1..FILE_10).rev() {
            fn_make_move_list(Square::from(file, rank));
        }
    }
}

/// 先手から見た盤上の歩の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_pawn<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination = &mut |to, _move_range| {
        Promoting::pawn_lance(
            us,
            to,
            fn_make_move_list,
            Some(MovePermission::from_pawn_or_lance(us)),
        )
    };

    for mobility in PieceType::P.mobility().iter() {
        push_piece_moves(Some(us), from, *mobility, fn_pass_destination);
    }
}

/// 先手から見た盤上の香の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_lance<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination = &mut |to, _move_range| {
        Promoting::pawn_lance(
            us,
            to,
            fn_make_move_list,
            Some(MovePermission::from_pawn_or_lance(us)),
        )
    };

    for mobility in PieceType::L.mobility().iter() {
        push_piece_moves(Some(us), from, *mobility, fn_pass_destination);
    }
}

/// 先手から見た盤上の桂の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_knight<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination = &mut |to, _move_range| {
        Promoting::knight(
            us,
            to,
            fn_make_move_list,
            Some(MovePermission::from_knight(us)),
        )
    };

    for mobility in PieceType::N.mobility().iter() {
        push_piece_moves(Some(us), from, *mobility, fn_pass_destination);
    }
}

/// 先手から見た盤上の銀の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_silver<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination =
        &mut |to, _move_range| Promoting::silver(us, from, to, fn_make_move_list);

    for mobility in PieceType::S.mobility().iter() {
        push_piece_moves(Some(us), from, *mobility, fn_pass_destination);
    }
}

/// 先手から見た盤上の金、と、杏、圭、全の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_gold<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool, // FnMut
{
    let fn_pass_destination = &mut |to, _move_range| {
        fn_make_move_list(to, Promotability::Deny, MoveRange::Adjacent, None)
    };

    for mobility in PieceType::G.mobility().iter() {
        push_piece_moves(Some(us), from, *mobility, fn_pass_destination);
    }
}

/// 盤上の玉の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_king<F1>(from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination = &mut |to, _move_range| {
        fn_make_move_list(to, Promotability::Deny, MoveRange::Adjacent, None)
    };

    for mobility in PieceType::K.mobility().iter() {
        // 先後同型
        push_piece_moves(None, from, *mobility, fn_pass_destination);
    }
}

/// 盤上の角の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_bishop<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination =
        &mut |to, _move_range| Promoting::bishop_rook(us, from, to, fn_make_move_list);
    for mobility in PieceType::B.mobility().iter() {
        push_piece_moves(
            None, //&Some(us),// 先後同型なのでは（＾～＾）？
            from,
            *mobility,
            fn_pass_destination,
        );
    }
}

/// 盤上の飛の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_rook<F1>(us: Phase, from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination =
        &mut |to, _move_range| Promoting::bishop_rook(us, from, to, fn_make_move_list);
    for mobility in PieceType::R.mobility().iter() {
        push_piece_moves(
            None, //&Some(us),// 先後同型なのでは（＾～＾）？
            from,
            *mobility,
            fn_pass_destination,
        );
    }
}

/// 盤上の馬の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_horse<F1>(from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    let fn_pass_destination =
        &mut |to, move_range| fn_make_move_list(to, Promotability::Deny, move_range, None);

    for mobility in PieceType::PB.mobility().iter() {
        // 先後同型（＾～＾）
        push_piece_moves(None, from, *mobility, fn_pass_destination);
    }
}

/// 盤上の竜の動けるマスだぜ☆（＾～＾）
///
/// # Arguments
///
/// * `from` - 移動元升だぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地、成れるか、動き方、移動できるかを受け取れだぜ☆（＾～＾）
fn gen_dragon<F1>(from: Square, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
{
    {
        let fn_pass_destination =
            &mut |to, move_range| fn_make_move_list(to, Promotability::Deny, move_range, None);

        for mobility in PieceType::PR.mobility().iter() {
            // 先後同型（＾～＾）
            push_piece_moves(None, from, *mobility, fn_pass_destination);
        }
    }
}

/// 先手から見た歩、香車の打てる面積だぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 後手視点にしたけりゃ us.turn() しろだぜ☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地を受け取れだぜ☆（＾～＾）
pub fn drop_pawn_lance<F1>(us: Phase, fn_make_move_list: &mut F1)
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
            fn_make_move_list(Square::from(file, rank));
        }
    }
}

/// 先手から見た桂馬の打てる面積だぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 手番☆（＾～＾）
/// * `fn_make_move_list` - 絶対番地を受け取れだぜ☆（＾～＾）
pub fn drop_knight<F1>(us: Phase, fn_make_move_list: &mut F1)
where
    F1: FnMut(Square),
{
    for rank in RANK_3..RANK_10 {
        for file in (FILE_1..FILE_10).rev() {
            let mut sq = Square::from(file, rank);
            if us == Phase::Second {
                sq = sq.rotate_180();
            }

            fn_make_move_list(sq);
        }
    }
}

/// 盤上の駒を指すぜ☆（＾～＾）
///
/// # Arguments
///
/// * `us` - 先手か後手か、関係ないか☆（＾～＾）先後同型なら None ☆（＾～＾）
/// * `start` - 移動元升☆（＾～＾）
/// * `square` - 升☆（＾～＾）
/// * `mobility` - 動き方☆（＾～＾）
/// * `fn_pass_destination` - 絶対番地を受け取れだぜ☆（＾～＾）
fn push_piece_moves<F1>(
    us: Option<Phase>,
    start: Square,
    mobility: Mobility,
    fn_pass_destination: &mut F1,
) where
    F1: FnMut(Square, MoveRange) -> bool,
{
    // 後手なら 180°ひっくり返す。 us が指定されていないとき、先後同型と見做して回転させません
    let angle = if let Some(us) = us {
        if us == Phase::First {
            mobility.angle
        } else {
            // 先後同型でない駒は、後手なら１８０°回転だぜ☆（＾～＾）
            mobility.angle.rotate180()
        }
    } else {
        // 先後同型だからそのままだぜ☆（＾～＾）
        mobility.angle
    };

    match mobility.move_range {
        // 飛、角、香、竜、馬
        MoveRange::Sliding => {
            let mut to = start;
            // 単にどっちに伸びるか相対角度を指定しているだけ（＾～＾）相対角度が 0 なら 西へ（＾～＾）
            let r = RelAdr::new(1, 0).rotate_ccw(angle).clone();
            // let r = RelAdr::new(1, 0).rotate_ccw(mobility.angle).clone();

            loop {
                // その方向へ１つ進めるぜ☆（＾～＾）
                to = to.go_forward(&r);
                if to.wall() {
                    break;
                }

                if fn_pass_destination(to, mobility.move_range) {
                    break;
                }
            }
        }
        // 桂馬
        MoveRange::Knight => {
            let mut to = start;

            // １つ進むだけ（＾～＾）
            to = to.go_forward(&angle.west_ccw_double_rank());
            if !to.wall() {
                fn_pass_destination(to, mobility.move_range);
            }
        }
        MoveRange::Adjacent => {
            let mut to = start;

            // １つ進むだけ（＾～＾）
            to = to.go_forward(&angle.west_ccw());
            if !to.wall() {
                fn_pass_destination(to, mobility.move_range);
            }
        }
    }
}

/// 機敏性。
#[derive(Clone, Copy, Debug)]
pub enum MoveRange {
    /// 隣へ１つ進む駒。
    Adjacent,
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
        if to.rank() < self.min_rank || self.max_rank < to.rank() {
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
    /// # Arguments
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
        F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
    {
        if Promoting::is_farthest_rank_from_friend(us, to) {
            // 自陣から見て一番奥の段
            callback(
                to,
                Promotability::Forced,
                MoveRange::Adjacent,
                move_permission,
            )
        } else if Promoting::is_second_third_farthest_rank_from_friend(us, to) {
            // 自陣から見て二番、三番目の奥の段
            callback(to, Promotability::Any, MoveRange::Adjacent, move_permission)
        } else {
            callback(
                to,
                Promotability::Deny,
                MoveRange::Adjacent,
                move_permission,
            )
        }
    }

    /// 桂のための、成れるか成れないか判定だぜ☆（＾～＾）！
    ///
    /// # Arguments
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
        F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
    {
        if Promoting::is_first_second_farthest_rank_from_friend(us, to) {
            callback(
                to,
                Promotability::Forced,
                MoveRange::Knight,
                move_permission,
            )
        } else if Promoting::is_third_farthest_rank_from_friend(us, to) {
            callback(to, Promotability::Any, MoveRange::Knight, move_permission)
        } else {
            callback(to, Promotability::Deny, MoveRange::Knight, move_permission)
        }
    }

    /// 銀のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 自陣から見て奥から１～３段目に入るときに成れます。元位置が３段目のときは、動けば成るか選べます。
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `from` -
    /// * `to` -
    /// * `callback` -
    fn silver<F1>(us: Phase, from: Square, to: Square, callback: &mut F1) -> bool
    where
        F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
    {
        if Promoting::is_third_farthest_rank_from_friend(us, from) {
            callback(to, Promotability::Any, MoveRange::Adjacent, None)
        } else if Promoting::is_opponent_region(us, to) {
            callback(to, Promotability::Any, MoveRange::Adjacent, None)
        } else {
            callback(to, Promotability::Deny, MoveRange::Adjacent, None)
        }
    }

    /// 角と飛のための、成れるか成れないか判定だぜ☆（＾～＾）！
    /// 非敵陣にいるとき、敵陣で成れます。敵陣にいるとき、どこでも成れます。
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `from` -
    /// * `to` -
    /// * `callback` -
    fn bishop_rook<F1>(us: Phase, from: Square, to: Square, callback: &mut F1) -> bool
    where
        F1: FnMut(Square, Promotability, MoveRange, Option<MovePermission>) -> bool,
    {
        if Promoting::is_opponent_region(us, from) || Promoting::is_opponent_region(us, to) {
            callback(to, Promotability::Any, MoveRange::Sliding, None)
        } else {
            callback(to, Promotability::Deny, MoveRange::Sliding, None)
        }
    }

    /// 自陣から見て、一番遠いの段
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `to` -
    fn is_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && to.rank() < RANK_2) || (us == Phase::Second && RANK_8 < to.rank())
    }
    /// 自陣から見て、一番目、２番目に遠いの段
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `to` -
    fn is_first_second_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && to.rank() < RANK_3) || (us == Phase::Second && RANK_7 < to.rank())
    }
    /// 自陣から見て、二番目、三番目に遠いの段
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `to` -
    fn is_second_third_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && RANK_1 < to.rank() && to.rank() < RANK_4)
            || (us == Phase::Second && RANK_6 < to.rank() && to.rank() < RANK_9)
    }
    /// 自陣から見て、三番目に遠いの段
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `to` -
    fn is_third_farthest_rank_from_friend(us: Phase, to: Square) -> bool {
        (us == Phase::First && to.rank() == RANK_3) || (us == Phase::Second && RANK_7 == to.rank())
    }
    /// 敵陣
    ///
    /// # Arguments
    ///
    /// * `us` -
    /// * `to` -
    fn is_opponent_region(us: Phase, to: Square) -> bool {
        (us == Phase::First && to.rank() < RANK_4) || (us == Phase::Second && RANK_6 < to.rank())
    }
}
