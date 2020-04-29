//! 一手詰め判定だぜ☆（＾～＾）
//! これが無いと、探索しなくていい枝を末端まで伸ばしてしまうぜ☆（＾～＾）

use crate::cosmic::playing::Game;
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::square::RelAdr;
use crate::cosmic::toy_box::{Location, PieceNum};
use crate::law::generate_move::Piece;
use crate::law::speed_of_light::Movility;
use crate::spaceship::equipment::Beam;

pub enum Mate1Result {
    /// 詰ませた☆（＾～＾）
    Checked,
    /// 空き王手になって、自玉が逆に王手回避放置になるぜ☆（＾～＾）！
    Counter,
    /// 詰ませてないぜ☆（＾～＾）
    NoMate,
    /// TODO おかしいぜ☆（＾～＾）
    Lioncatch,
}

pub struct Mate1 {}
impl Mate1 {
    /// まず、王手してるか判定して、王手していたらこれを呼べだぜ☆（＾～＾）
    pub fn can_evasion(_game: &Game) -> bool {
        /*
        // 相手の手番☆（＾～＾）
        let opponent = game.history.get_friend();
        // 敵玉の場所☆（＾～＾）
        let opponent_king_adr = match game.board.location_at(match opponent {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => panic!(Beam::trouble(
                "(Err.37) なんで敵玉が持ち駒になってて、回避判定してんだぜ☆（＾～＾）！"
            )),
            Location::Busy => panic!(Beam::trouble(
                "(Err.51) なんで敵玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };

        // 自分の手番☆（＾～＾）
        let friend = opponent.turn();
        // 敵玉の８方向☆（＾～＾）
        let sign = if friend == Phase::Second { -1 } else { 1 };
        // Beam::shoot(&format!("Mate1 | sign={}", sign));

        // TODO speed of light に入れたいぜ☆（＾～＾）
        let recipes = [
            RelAdr::new(1, sign * 0),   // 西
            RelAdr::new(1, sign * 1),   // 南西
            RelAdr::new(0, sign * 1),   // 南
            RelAdr::new(-1, sign * 1),  // 南東
            RelAdr::new(-1, sign * 0),  // 東
            RelAdr::new(-1, sign * -1), // 北東
            RelAdr::new(0, sign * -1),  // 北
            RelAdr::new(1, sign * -1),  // 北西
                                        // TODO 飛び利きにも対応したいぜ☆（＾～＾）
        ];

        let mut can_evasion = false;
        for recipe in &recipes {
            let mut cur = opponent_king_adr.clone();
            // Beam::shoot(&format!("Mate1 | cur={:?}", cur));

            if cur.offset(&recipe).legal_cur() {
                // Beam::shoot(&format!("Mate1 | legal cur={:?}", cur));
                let piece = game.board.piece_at(&cur);
                if let Some(piece_val) = piece {
                    // Beam::shoot(&format!("Mate1 | piece={:?}", piece_val));
                    // Beam::shoot(&format!(
                    //     "Mate1 | piece.phase={}",
                    //     piece_val.meaning.phase()
                    // ));
                    // Beam::shoot(&format!(
                    //     "Mate1 | contains recipe={}",
                    //     piece_val.meaning.r#type().movility().contains(&recipe.1)
                    // ));
                    if piece_val.meaning.phase() == opponent {
                        // こっちには避けれないぜ☆（＾～＾）
                        break;
                    }

                    // if game.board.control
                    // match piece_val.meaning.phase() {
                    //     Phase::First
                    // }
                    // if piece_val.meaning.r#type().movility().contains(&recipe.1)
                    // {
                    //     // Beam::shoot("Mate1 | mate!");
                    //     // 敵玉に自駒が当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                    //     return Mate1Result::Checked;
                    // }
                }
            }
        }

        can_evasion
        */
        true
    }

    /// このメソッドが呼び出されるのは、１手指した直後だから、局面は相手視点になっているぜ☆（＾～＾）
    /// 玉が詰んでたら、１手指した方の勝ちだぜ☆（＾～＾） ただし、そいつに空き王手がかかる形では この手は使えないぜ☆（＾～＾）
    pub fn was_checked(game: &Game) -> Mate1Result {
        // 相手の手番☆（＾～＾）
        let opponent = game.history.get_friend();
        // Beam::shoot(&format!("Mate1 | opponent={}", opponent));
        /*
        // 自玉の場所☆（＾～＾）
        let friend_king_adr = match game.board.location_at(match friend {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(adr) => panic!(Beam::trouble(
                "(Err.21) なんで自玉が持ち駒になってて、１手詰め判定してんだぜ☆（＾～＾）！"
            )),
            Location::Busy => panic!(Beam::trouble(
                "(Err.25) なんで自玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };
        */
        // 自分の手番☆（＾～＾）
        let friend = opponent.turn();
        // Beam::shoot(&format!("Mate1 | friend={}", friend));
        // 敵玉の場所☆（＾～＾）
        let opponent_king_adr = match game.board.location_at(match opponent {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => {
                // TODO らいおんキャッチするのはおかしいぜ☆（＾～＾）！
                return Mate1Result::Lioncatch;
                // panic!(Beam::trouble(
                //     "(Err.48) なんで敵玉が持ち駒になってて、１手詰め判定してんだぜ☆（＾～＾）！"
                // ))
            }
            Location::Busy => panic!(Beam::trouble(
                "(Err.51) なんで敵玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };
        // Beam::shoot(&format!(
        //     "Mate1 | opponent_king_adr={:?}",
        //     opponent_king_adr
        // ));
        // TODO ピンされている駒が動いたことで、空き王手になる可能性があるぜ☆（＾～＾）

        // 自駒が敵玉に当たってるなら、取れるな☆（＾～＾）
        let sign = if friend == Phase::Second { -1 } else { 1 };
        // Beam::shoot(&format!("Mate1 | sign={}", sign));

        // TODO speed of light に入れたいぜ☆（＾～＾）
        let recipes = [
            (RelAdr::new(-1, sign * 2), Movility::Knight),  // 桂馬
            (RelAdr::new(1, sign * 2), Movility::Knight),   // 桂馬
            (RelAdr::new(1, sign * 0), Movility::SideBack), // 西
            (RelAdr::new(1, sign * 1), Movility::BackDiagonally), // 南西
            (RelAdr::new(0, sign * 1), Movility::SideBack), // 南
            (RelAdr::new(-1, sign * 1), Movility::BackDiagonally), // 南東
            (RelAdr::new(-1, sign * 0), Movility::SideBack), // 東
            (RelAdr::new(-1, sign * -1), Movility::FrontDiagonally), // 北東
            (RelAdr::new(0, sign * -1), Movility::Front),   // 北
            (RelAdr::new(1, sign * -1), Movility::FrontDiagonally), // 北西
                                                            // TODO 飛び利きにも対応したいぜ☆（＾～＾）
        ];

        for recipe in &recipes {
            let mut cur = opponent_king_adr.clone();
            // Beam::shoot(&format!("Mate1 | cur={:?}", cur));

            if cur.offset(&recipe.0).legal_cur() {
                // Beam::shoot(&format!("Mate1 | legal cur={:?}", cur));
                let piece = game.board.piece_at(&cur);
                if let Some(piece_val) = piece {
                    // Beam::shoot(&format!("Mate1 | piece={:?}", piece_val));
                    // Beam::shoot(&format!(
                    //     "Mate1 | piece.phase={}",
                    //     piece_val.meaning.phase()
                    // ));
                    // Beam::shoot(&format!(
                    //     "Mate1 | contains recipe={}",
                    //     piece_val.meaning.r#type().movility().contains(&recipe.1)
                    // ));
                    if piece_val.meaning.phase() == friend
                        && piece_val.meaning.r#type().movility().contains(&recipe.1)
                    {
                        // Beam::shoot("Mate1 | mate!");
                        // 敵玉に自駒が当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                        return Mate1Result::Checked;
                    }
                }
            }
        }

        Mate1Result::NoMate
    }
}
