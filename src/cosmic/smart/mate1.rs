//! 一手詰め判定だぜ☆（＾～＾）
//! これが無いと、探索しなくていい枝を末端まで伸ばしてしまうぜ☆（＾～＾）

use crate::cosmic::playing::Game;
use crate::cosmic::recording::Phase;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::cosmic::smart::square::RelAdr;
use crate::cosmic::toy_box::{Location, PieceNum};
use crate::law::speed_of_light::Movility;
use crate::spaceship::equipment::Beam;

pub struct Mate1 {
    /// 自分の手番☆（＾～＾）
    friend: Phase,
    /// 相手の手番☆（＾～＾）
    opponent: Phase,
    /// 自玉の場所☆（＾～＾）
    friend_king_adr: AbsoluteAddress,
    /// 敵玉の場所☆（＾～＾）
    opponent_king_adr: AbsoluteAddress,
    /// 王手を掛けている駒の背番号だぜ☆（＾～＾）
    pub checkers: Option<Vec<PieceNum>>,
    /// 動かしてはいけない駒の背番号の一覧を作るぜ☆（＾～＾）
    pub pinned_pieces: Option<Vec<PieceNum>>,
    /// TODO おかしいぜ☆（＾～＾）
    pub lioncatch: bool,
    /// TODO 空き王手になって、自玉が逆に王手回避放置になるぜ☆（＾～＾）！
    pub counter: bool,
}
impl Mate1 {
    pub fn new(game: &Game) -> Self {
        let friend = game.history.get_friend();
        Mate1 {
            friend: friend,
            opponent: friend.turn(),
            friend_king_adr: AbsoluteAddress::default(),
            opponent_king_adr: AbsoluteAddress::default(),
            checkers: None,
            pinned_pieces: None,
            lioncatch: false,
            counter: false,
        }
    }

    pub fn end(&mut self) {
        if let Some(pinned_pieces_val) = &mut self.pinned_pieces {
            if let Some(checkers_val) = &mut self.checkers {
                checkers_val.retain(|x| !(*pinned_pieces_val).contains(&x))
            }
        }
    }
    pub fn init(&mut self, game: &Game) -> &mut Self {
        // 自玉の場所☆（＾～＾）
        self.friend_king_adr = match game.board.location_at(match self.friend {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => {
                // TODO らいおんキャッチするのはおかしいぜ☆（＾～＾）！
                self.lioncatch = true;
                return self;
                // panic!(Beam::trouble(
                //     "(Err.48) なんで敵玉が持ち駒になってて、１手詰め判定してんだぜ☆（＾～＾）！"
                // ))
            }
            Location::Busy => panic!(Beam::trouble(
                "(Err.51) なんで敵玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };

        // 敵玉の場所☆（＾～＾）
        self.opponent_king_adr = match game.board.location_at(match self.opponent {
            Phase::First => PieceNum::King1,
            Phase::Second => PieceNum::King2,
        }) {
            Location::Board(adr) => adr,
            Location::Hand(_adr) => {
                // TODO らいおんキャッチするのはおかしいぜ☆（＾～＾）！
                self.lioncatch = true;
                return self;
                // panic!(Beam::trouble(
                //     "(Err.48) なんで敵玉が持ち駒になってて、１手詰め判定してんだぜ☆（＾～＾）！"
                // ))
            }
            Location::Busy => panic!(Beam::trouble(
                "(Err.51) なんで敵玉が作業中なんだぜ☆（＾～＾）！"
            )),
        };
        self
    }
    /// TODO 動かしてはいけない駒の一覧を作るぜ☆（＾～＾）
    pub fn pinned_pieces(&mut self, game: &Game) -> &mut Self {
        // 自玉の８方向を調べようぜ☆（＾～＾）
        // 味方の駒、相手の香飛角の順で駒が現れたらピン確定だぜ☆（＾～＾）
        // TODO speed of light に入れたいぜ☆（＾～＾）
        let sign = if self.friend == Phase::Second { -1 } else { 1 };
        let recipes = [
            (RelAdr::new(1, sign * 0), Movility::SideBackSlider), // 西
            (RelAdr::new(1, sign * 1), Movility::SlideDiagonally), // 南西
            (RelAdr::new(0, sign * 1), Movility::SideBackSlider), // 南
            (RelAdr::new(-1, sign * 1), Movility::SlideDiagonally), // 南東
            (RelAdr::new(-1, sign * 0), Movility::SideBackSlider), // 東
            (RelAdr::new(-1, sign * -1), Movility::SlideDiagonally), // 北東
            (RelAdr::new(0, sign * -1), Movility::FrontSlider),   // 北
            (RelAdr::new(1, sign * -1), Movility::SlideDiagonally), // 北西
        ];

        let mut pinned_pieces = Vec::<PieceNum>::new();
        for recipe in &recipes {
            let mut cur = self.friend_king_adr.clone();
            let mut friend_piece = None;

            for _i in 0..8 {
                if cur.offset(&recipe.0).legal_cur() {
                    let any_piece = game.board.piece_at(&cur);
                    if let Some(any_piece_val) = any_piece {
                        if let None = friend_piece {
                            // 味方の駒か☆（＾～＾）？
                            if any_piece_val.meaning.phase() == self.friend {
                                // そうだぜ☆（＾～＾）
                                friend_piece = any_piece;
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
                                    pinned_pieces.push(any_piece_val.num);
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
    /// ただし、自玉に空き王手がかかる形では この手は使えないぜ☆（＾～＾）
    pub fn checkers(&mut self, game: &Game) -> &mut Self {
        // TODO speed of light に入れたいぜ☆（＾～＾）
        let sign = if self.friend == Phase::Second { -1 } else { 1 };
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
        ];

        let mut checkers = Vec::<PieceNum>::new();
        // 王手を掛けている駒を全部挙げろだぜ☆（＾～＾）
        for recipe in &recipes {
            let mut cur = self.opponent_king_adr.clone();
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
                    if piece_val.meaning.phase() == self.friend
                        && piece_val.meaning.r#type().movility().contains(&recipe.1)
                    {
                        // Beam::shoot("Mate1 | mate!");
                        // 敵玉に自駒が当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                        checkers.push(piece_val.num);
                    }
                }
            }
        }

        // TODO スライダー駒も判定しようぜ☆（＾～＾）？
        // TODO speed of light に入れたいぜ☆（＾～＾）
        let recipes = [
            (RelAdr::new(1, sign * 0), Movility::SideBackSlider), // 西
            (RelAdr::new(1, sign * 1), Movility::SlideDiagonally), // 南西
            (RelAdr::new(0, sign * 1), Movility::SideBackSlider), // 南
            (RelAdr::new(-1, sign * 1), Movility::SlideDiagonally), // 南東
            (RelAdr::new(-1, sign * 0), Movility::SideBackSlider), // 東
            (RelAdr::new(-1, sign * -1), Movility::SlideDiagonally), // 北東
            (RelAdr::new(0, sign * -1), Movility::FrontSlider),   // 北
            (RelAdr::new(1, sign * -1), Movility::SlideDiagonally), // 北西
        ];

        for recipe in &recipes {
            let mut cur = self.opponent_king_adr.clone();

            for i in 0..8 {
                if cur.offset(&recipe.0).legal_cur() {
                    let piece = game.board.piece_at(&cur);
                    if let Some(piece_val) = piece {
                        if piece_val.meaning.phase() == self.friend
                            && piece_val.meaning.r#type().movility().contains(&recipe.1)
                        {
                            if i != 0 {
                                // Beam::shoot("Mate1 | mate!");
                                // 敵玉に自駒スライダーが当たってるぜ☆（＾～＾）！ まず王手は確定だぜ☆（＾～＾）
                                checkers.push(piece_val.num);
                            }
                        }
                        // なんか駒に当たったよな☆（＾～＾） スライダー終わり☆（＾～＾）
                        break;
                    }
                }
            }
        }

        if !checkers.is_empty() {
            self.checkers = Some(checkers);
        }

        self
    }
}
