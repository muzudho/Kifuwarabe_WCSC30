//! Static exchange evaluation

use crate::cosmic::daydream::Value;
use crate::cosmic::playing::Game;
use crate::cosmic::smart::square::AbsoluteAddress;
/*
use crate::cosmic::smart::square::RelAdr;
use crate::law::generate_move::Piece;
use crate::law::speed_of_light::Movility;
use crate::spaceship::equipment::Beam;
// */

pub struct SEE {}
impl SEE {
    /// 葉で駒を取ったら、取り返されるのも考慮しないとな☆（＾～＾）
    ///
    /// 価値の低い駒から順に使って、取りに行けだぜ☆（＾～＾）
    pub fn go(_game: &Game, _adr: &AbsoluteAddress) -> Value {
        /*
        // この駒☆（＾～＾）
        let mut current_target_piece = game.board.piece_at(adr).unwrap();
        let mut centi_pawn = 0;

        // この駒の西に相手の駒があって、それが この駒に利いているなら、取りにくると思おうぜ☆（＾～＾）
        let recipes = [
            (RelAdr::new(1, 0), Movility::SideBack),          // 西
            (RelAdr::new(1, 1), Movility::BackDiagonally),    // 南西
            (RelAdr::new(0, 1), Movility::SideBack),          // 南
            (RelAdr::new(-1, 1), Movility::BackDiagonally),   // 南東
            (RelAdr::new(-1, 0), Movility::SideBack),         // 東
            (RelAdr::new(-1, -1), Movility::FrontDiagonally), // 北東
            (RelAdr::new(0, -1), Movility::Front),            // 北
            (RelAdr::new(1, -1), Movility::FrontDiagonally),  // 北西
                                                              // TODO 飛び利きにも対応したいぜ☆（＾～＾）
        ];
        // 移動先升に利きのある駒が無くなるまで繰り返すぜ☆（＾～＾）
        for i in 0..102 {
            let mut next_target_piece = None;
            // TODO 相手の駒が、自分の駒を取る全てのケースを一覧しろだぜ☆（＾～＾）
            let attackers = Vec::<Piece>::new();
            for recipe in &recipes {
                let mut cur = adr.clone();
                if cur.offset(&recipe.0).legal_cur() {
                    let piece = game.board.piece_at(&cur);
                    if let Some(piece_val) = piece {
                        if piece_val.meaning.phase() != current_target_piece.meaning.phase()
                            || piece_val.meaning.r#type().movility().contains(&recipe.1)
                        {
                            // 敵の駒も西に動けるんだったら、利かされているぜ☆（＾～＾）
                            // 自分の駒が取られるということだぜ☆（＾～＾）
                            attackers.push(piece_val);
                        }
                    }
                }
            }

            for recipe in &recipes {
                let mut cur = adr.clone();
                if cur.offset(&recipe.0).legal_cur() {
                    let piece = game.board.piece_at(&cur);
                    if let Some(piece_val) = piece {
                        if piece_val.meaning.phase() != current_target_piece.meaning.phase()
                            || piece_val.meaning.r#type().movility().contains(&recipe.1)
                        {
                            // 敵の駒も西に動けるんだったら、利かされているぜ☆（＾～＾）
                            // 自分の駒が取られるということだぜ☆（＾～＾）
                            centi_pawn -= current_target_piece
                                .meaning
                                .hand_address()
                                .r#type()
                                .captured_value();
                            next_target_piece = piece;
                            game.do_move();
                            break;
                        }
                    }
                }
            }
            if let Some(piece) = next_target_piece {
                current_target_piece = piece;
            } else {
                break;
            }

            if 100 < i {
                panic!(Beam::trouble(
                    "(Err.61) SEEが終わらんぜ☆（＾～＾）バグってんじゃねーの☆（＾～＾）？"
                ))
            }
        }
        Value::CentiPawn(centi_pawn)
        // */
        Value::CentiPawn(0)
    }
}
