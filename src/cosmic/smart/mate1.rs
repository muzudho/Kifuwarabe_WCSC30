//! 一手詰め判定だぜ☆（＾～＾）
//! これが無いと、探索しなくていい枝を末端まで伸ばしてしまうぜ☆（＾～＾）

use crate::cosmic::playing::Game;
use crate::cosmic::recording::Movement;
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::cosmic::smart::square::RelAdr;
use crate::cosmic::toy_box::{Location, PieceNum};
use crate::law::generate_move::{Way, Ways};
use crate::law::speed_of_light::Movility;
use crate::spaceship::equipment::{Beam, Log};

/// これは一手詰め判定ではなく、ライオンキャッチ判定なのでは☆（＾～＾）？
pub struct Lioncatch {
    /// 自分の手番☆（＾～＾）
    friend: Phase,
    /// 相手の手番☆（＾～＾）
    opponent: Phase,
    /// 自玉の場所☆（＾～＾）
    friend_king_adr: AbsoluteAddress,
    /// 敵玉の場所☆（＾～＾）
    opponent_king_adr: AbsoluteAddress,
    /// 王手の指し手一覧だぜ☆（＾～＾）
    pub checks: Ways,
    /// 王手を掛けている駒の背番号だぜ☆（＾～＾）
    // pub checkers: Option<Vec<PieceNum>>,
    /// 動かしてはいけない駒の背番号の一覧を作るぜ☆（＾～＾）
    pub pinned_pieces: Option<Vec<PieceNum>>,
    /// 上下反転させるのに使うぜ☆（＾～＾）
    pub sign: isize,
}
impl Lioncatch {
    pub fn new(game: &Game) -> Self {
        let friend = game.history.get_friend();
        Lioncatch {
            friend: friend,
            opponent: friend.turn(),
            friend_king_adr: AbsoluteAddress::default(),
            opponent_king_adr: AbsoluteAddress::default(),
            checks: Ways::new(),
            // checkers: None,
            pinned_pieces: None,
            sign: 1,
        }
    }

    pub fn init(&mut self, game: &Game) -> &mut Self {
        // 自玉の場所☆（＾～＾）
        self.friend_king_adr = match game.board.location_at(match self.friend {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => panic!(Beam::trouble(
                "(Err.62) なんで自玉が持ち駒になってて、１手詰め判定してんだぜ☆（＾～＾）！"
            )),
            Location::Busy => panic!(Beam::trouble(
                "(Err.66) なんで自玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };

        // 敵玉の場所☆（＾～＾）
        self.opponent_king_adr = match game.board.location_at(match self.opponent {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => panic!(Beam::trouble(
                "(Err.48) なんで敵玉が持ち駒になってて、１手詰め判定してんだぜ☆（＾～＾）！"
            )),
            Location::Busy => panic!(Beam::trouble(
                "(Err.83) なんで敵玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };

        self.sign = if self.friend == Phase::Second { -1 } else { 1 };

        self
    }

    /// 動かしてはいけない駒の一覧を作るぜ☆（＾～＾）
    pub fn pinned_pieces(&mut self, game: &Game) -> &mut Self {
        // 自玉の８方向を調べようぜ☆（＾～＾）
        // 味方の駒、相手の香飛角の順で駒が現れたらピン確定だぜ☆（＾～＾）
        // TODO speed of light に入れたいぜ☆（＾～＾）
        // (自玉からスキャンする方向, 敵駒から見て自玉の方を向いているモビリティ)
        let mut recipes = [
            (RelAdr::new(1, 0), Movility::SideBackSlider),    // 西
            (RelAdr::new(1, 1), Movility::SlideDiagonally),   // 南西
            (RelAdr::new(0, 1), Movility::SideBackSlider),    // 南
            (RelAdr::new(-1, 1), Movility::SlideDiagonally),  // 南東
            (RelAdr::new(-1, 0), Movility::SideBackSlider),   // 東
            (RelAdr::new(-1, -1), Movility::SlideDiagonally), // 北東
            (RelAdr::new(0, -1), Movility::FrontSlider),      // 北
            (RelAdr::new(1, -1), Movility::SlideDiagonally),  // 北西
        ];

        // 上下反転ではなく、１８０°回転しろだぜ☆（＾～＾）
        if self.sign < 1 {
            for recipe in &mut recipes {
                recipe.0.rotate_180();
            }
        }

        let mut pinned_pieces = Vec::<PieceNum>::new();
        for recipe in &recipes {
            let mut cur = self.friend_king_adr.clone();
            let mut friend_piece = None;

            for _i in 0..8 {
                if !cur.offset(&recipe.0).wall() {
                    let any_piece = game.board.piece_at(&cur);
                    if let Some(any_piece_val) = any_piece {
                        if let None = friend_piece {
                            // 味方の駒か☆（＾～＾）？
                            if any_piece_val.meaning.phase() == self.friend {
                                // そうだぜ☆（＾～＾）
                                friend_piece = Some(any_piece_val);
                            } else {
                                // おわり☆（＾～＾）
                                break;
                            }
                        } else {
                            // 相手の香飛角か☆（＾～＾）？
                            if any_piece_val.meaning.phase() == self.opponent {
                                if any_piece_val
                                    .meaning
                                    .r#type()
                                    .movility()
                                    .contains(&recipe.1)
                                {
                                    // そうだぜ☆（＾～＾）ピンされている方確定だな☆（＾～＾）
                                    pinned_pieces.push(friend_piece.unwrap().num);
                                } else {
                                    // おわり☆（＾～＾）
                                    break;
                                }
                            } else {
                                // おわり☆（＾～＾）
                                break;
                            }
                        }
                    }
                }
            }
        }

        if !pinned_pieces.is_empty() {
            self.pinned_pieces = Some(pinned_pieces);
        }

        self
    }

    /// 相手玉を取れる駒（checkers）たちを調べるぜ☆
    /// ピンされている駒を調べたあとで使えだぜ☆（＾～＾）
    /// ただし、自玉に空き王手がかかる形では この手は使えないぜ☆（＾～＾）
    pub fn checkers(&mut self, game: &Game) -> &mut Self {
        // TODO speed of light に入れたいぜ☆（＾～＾）
        let mut recipes = [
            // (相手玉から見て隣へ, 自駒から見て相手玉に当たっている方)
            (RelAdr::new(-1, 2), Movility::Knight),  // 桂馬
            (RelAdr::new(1, 2), Movility::Knight),   // 桂馬
            (RelAdr::new(1, 0), Movility::SideBack), // 西
            (RelAdr::new(1, 1), Movility::FrontDiagonally), // 南西
            (RelAdr::new(0, 1), Movility::Front),    // 南
            (RelAdr::new(-1, 1), Movility::FrontDiagonally), // 南東
            (RelAdr::new(-1, 0), Movility::SideBack), // 東
            (RelAdr::new(-1, -1), Movility::BackDiagonally), // 北東
            (RelAdr::new(0, -1), Movility::SideBack), // 北
            (RelAdr::new(1, -1), Movility::BackDiagonally), // 北西
        ];

        // 上下反転ではなく、１８０°回転しろだぜ☆（＾～＾）
        if self.sign < 1 {
            for recipe in &mut recipes {
                recipe.0.rotate_180();
            }
        }
        // 王手を掛けている駒を全部挙げろだぜ☆（＾～＾）
        for recipe in &recipes {
            // 相手玉をスタート地点にするぜ☆（＾～＾）
            let mut cur = self.opponent_king_adr.clone();
            Log::write(&format!("cur1={:?}", cur));
            if !cur.offset(&recipe.0).wall() {
                Log::write(&format!("cur2={:?}", cur));
                // 1つ隣に駒があるか確認だぜ☆（＾～＾）
                if let Some(any_piece_val) = game.board.piece_at(&cur) {
                    if any_piece_val.meaning.phase() == self.friend
                        && any_piece_val
                            .meaning
                            .r#type()
                            .movility()
                            .contains(&recipe.1)
                    {
                        if let Some(pinned_pieces_val) = &mut self.pinned_pieces {
                            if pinned_pieces_val.contains(&any_piece_val.num) {
                                // ピンされてる駒だった☆（＾～＾）動かせないぜ☆（＾～＾）！
                            } else {
                                // 相手玉に自駒が当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                                self.checks.push(&Way::new(
                                    Movement::new(Some(cur), self.opponent_king_adr, false, None),
                                    Some(any_piece_val),
                                ));
                            }
                        } else {
                            // 相手玉に自駒が当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                            self.checks.push(&Way::new(
                                Movement::new(Some(cur), self.opponent_king_adr, false, None),
                                Some(any_piece_val),
                            ));
                        }
                    }
                }
            }
        }

        // スライダー駒も判定しようぜ☆（＾～＾）？
        // TODO speed of light に入れたいぜ☆（＾～＾）
        let mut recipes = [
            // (相手玉から見て隣へ, 自駒から見て相手玉に当たっている方)
            (RelAdr::new(1, 0), Movility::SideBackSlider), // 西
            (RelAdr::new(1, 1), Movility::SlideDiagonally), // 南西
            (RelAdr::new(0, 1), Movility::FrontSlider),    // 南
            (RelAdr::new(-1, 1), Movility::SlideDiagonally), // 南東
            (RelAdr::new(-1, 0), Movility::SideBackSlider), // 東
            (RelAdr::new(-1, -1), Movility::SlideDiagonally), // 北東
            (RelAdr::new(0, -1), Movility::SideBackSlider), // 北
            (RelAdr::new(1, -1), Movility::SlideDiagonally), // 北西
        ];

        // 上下反転ではなく、１８０°回転しろだぜ☆（＾～＾）
        if self.sign < 1 {
            for recipe in &mut recipes {
                recipe.0.rotate_180();
            }
        }

        for recipe in &recipes {
            // 相手玉をスタート地点にするぜ☆（＾～＾）
            let mut cur = self.opponent_king_adr.clone();

            for i in 0..8 {
                if !cur.offset(&recipe.0).wall() {
                    // 1つ隣になんか駒があるか確認だぜ☆（＾～＾）
                    if let Some(any_piece_val) = game.board.piece_at(&cur) {
                        // それがスライディング自駒か確認だぜ☆（＾～＾）
                        if any_piece_val.meaning.phase() == self.friend
                            && any_piece_val
                                .meaning
                                .r#type()
                                .movility()
                                .contains(&recipe.1)
                        {
                            if let Some(pinned_pieces_val) = &mut self.pinned_pieces {
                                if pinned_pieces_val.contains(&any_piece_val.num) {
                                    // ピンされてる駒だった☆（＾～＾）動かせないぜ☆（＾～＾）！
                                } else {
                                    // 隣接している駒は、さっき入れたはずだぜ☆（＾～＾）
                                    if i != 0 {
                                        // 相手玉に自駒スライダーが当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                                        self.checks.push(&Way::new(
                                            Movement::new(
                                                Some(cur),
                                                self.opponent_king_adr,
                                                false,
                                                None,
                                            ),
                                            Some(any_piece_val),
                                        ));
                                    }
                                }
                            } else {
                                // ピンされている駒はないんだって☆（＾～＾）！
                                // 隣接している駒は、さっき入れたはずだぜ☆（＾～＾）
                                if i != 0 {
                                    // 相手玉に自駒スライダーが当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                                    self.checks.push(&Way::new(
                                        Movement::new(
                                            Some(cur),
                                            self.opponent_king_adr,
                                            false,
                                            None,
                                        ),
                                        Some(any_piece_val),
                                    ));
                                }
                            }
                        }
                        // なんか駒に当たったよな☆（＾～＾） スライダー終わり☆（＾～＾）
                        break;
                    }
                }
            }
        }

        self
    }
}
