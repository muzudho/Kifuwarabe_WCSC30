//! Static exchange evaluation

use crate::cosmic::playing::Game;
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::cosmic::smart::square::RelAdr;

pub struct SEE {}
impl SEE {
    /// 葉で駒を取ったら、取り返されるのも考慮しないとな☆（＾～＾）
    pub fn go(game: &Game, adr: &AbsoluteAddress) {
        // この駒☆（＾～＾）
        let this_piece = game.board.piece_at(adr).unwrap();
        let friend = this_piece.0.phase();

        // この駒の西に相手の駒があって、それが この駒に利いているなら、取りにくると思おうぜ☆（＾～＾）
        let mut cur = *adr;
        // 西
        cur.offset(&RelAdr::new(1, 0));
        if cur.legal_cur() {
            let piece = game.board.piece_at(&cur);
            if let Some(piece_val) = piece {
                if piece_val.0.phase() != friend {
                    // 敵の駒も西に動けるんだったら、利かされているぜ☆（＾～＾）
                }
            }
        }
        /*
        // 移動先升に利きのある駒が無くなるまで繰り返すぜ☆（＾～＾）
        loop {
            if cur.legal_cur() {
            }
            break;
        }
        */
    }
}
