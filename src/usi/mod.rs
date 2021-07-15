use crate::usi::chiyuri::Chiyuri;
use crate::usi::kifuwarabe::Kifuwarabe;
use crate::Universe;

mod chiyuri;
mod kifuwarabe;
pub mod yumemi;

pub fn main_loop(universe: &mut Universe) {
    loop {
        let (line, len, starts) = Kifuwarabe::catch_the_message();

        if len == 26 && line == "position startpos moves *0" {
            // 将棋所の連続対局中に
            // 相手が 時間切れを知らずに bestmove を返すと、
            // 将棋所は `isready` など次の対局が始まっている最中に
            // `position startpos moves *0` を返してくる。
            // この `*0` をパースできずに落ちることがあるので、無視するぜ（＾～＾）
            continue;
        }

        if len == 0 {
            // 任せろだぜ☆（＾～＾）
            Chiyuri::len0(universe);
        // 文字数の長いものからチェック
        } else if 10 <= len && &line[starts..10] == "usinewgame" {
            Kifuwarabe::usinewgame(universe);
        } else if line.starts_with("position") {
            Kifuwarabe::position(universe, &line);
        } else if 7 <= len && &line[starts..7] == "isready" {
            Kifuwarabe::isready();
        } else if 4 <= len && &line[starts..4] == "quit" {
            // ループを抜けて終了
            break;
        } else if 15 <= len && &line[starts..15] == "setoption name " {
            Kifuwarabe::setoption_name(universe, &line);
        } else if 3 <= len && &line[starts..3] == "usi" {
            Kifuwarabe::usi();
        } else if 2 <= len && &line[starts..2] == "go" {
            Kifuwarabe::go(universe);
        } else if 8 <= len && &line[starts..8] == "gameover" {
            // gameover win
            // gameover lose
            // gameover draw
            // 時間切れのときなど、将棋所から このメッセージがくるぜ（＾～＾）
            // TODO 時間切れと知らず指し手を返すと 将棋所の連続対局で不具合を起こすから、指し手は返すなだぜ（＾～＾）
            // といっても、そんなことは難しい（＾～＾）
        } else {
            help_chiyuri(&line, len, starts, universe);
        }
    } //loop
}

/// 独自コマンド☆（＾～＾）
fn help_chiyuri(line: &str, len: usize, starts: usize, universe: &mut Universe) {
    // D
    if 3 <= len && &line[starts..3] == "do " {
        Chiyuri::do_(universe, line, len, starts);
    // G
    } else if 7 <= len && &line[starts..7] == "genmove" {
        Chiyuri::genmove(&universe.game);
    // H
    } else if 8 <= len && &line[starts..8] == "how-much" {
        Chiyuri::how_much(line);
    } else if 4 <= len && &line[starts..4] == "hash" {
        Chiyuri::hash(universe);
    } else if 6 <= len && &line[starts..6] == "record" {
        // 棋譜（指し手）の表示
        Chiyuri::record(universe);
    /* TODO
    } else if 3 < len && &line[starts..4] == "kiki" {
        Chiyuri::kiki(universe);
    */
    // L
    } else if 6 <= len && &line[starts..6] == "list40" {
        Chiyuri::list40(universe);
    // P
    } else if 4 <= len && &line[starts..4] == "pos0" {
        Chiyuri::pos0(universe);
    } else if 3 <= len && &line[starts..3] == "pos" {
        Chiyuri::pos(universe);
    // S
    } else if 8 <= len && &line[starts..8] == "startpos" {
        Chiyuri::startpos(universe);
    // R
    } else if 4 <= len && &line[starts..4] == "rand" {
        Chiyuri::rand();
    // S
    } else if 4 <= len && &line[starts..4] == "same" {
        Chiyuri::same(universe);
    // T
    } else if 11 <= len && &line[starts..11] == "teigi::conv" {
        Chiyuri::teigi_conv();
    // U
    } else if 4 <= len && &line[starts..4] == "undo" {
        Chiyuri::undo(universe);
    }
}
