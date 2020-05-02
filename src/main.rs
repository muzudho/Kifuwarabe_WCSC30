//!
//! きふわらべＷＣＳＣ３０
//!
//! これは、最初に実行されるファイルだぜ☆（＾～＾）
//!

// extern crate は、 main.rs か lib.rs の冒頭にまとめろだぜ☆（＾～＾）
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate atoi;
extern crate num_derive;
extern crate num_traits;

// Rust言語の mod や ソース置き場の説明
//     「Rust のモジュールシステム」
//      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
//
// 使いたい ディレクトリー名を pub mod しろだぜ☆（＾～＾）
// 別のアプリにも見えるようにしたけりゃ pub mod にしろだぜ☆（＾～＾）
mod config;
mod cosmic;
mod law;
mod spaceship;

use crate::cosmic::universe::Universe;
use crate::spaceship::crew::{Chiyuri, Kifuwarabe, Yumemi};

fn main() {
    // 宇宙☆（＾～＾）変化するぜ☆（＾～＾）
    let mut universe: Universe = Universe::default();

    // ビッグバン
    universe.big_bang();

    // 「何が見えんの？」
    Yumemi::look_into_the_telescope();

    main_loop(&mut universe);
    // [Ctrl]+[C] で強制終了
}

fn main_loop(universe: &mut Universe) {
    loop {
        let (line, len, starts) = Kifuwarabe::catch_the_message();

        if len == 0 {
            // 任せろだぜ☆（＾～＾）
            Chiyuri::len0(universe);
        // 文字数の長いものからチェック
        } else if 9 < len && &line[starts..10] == "usinewgame" {
            Kifuwarabe::usinewgame(universe);
        } else if line.starts_with("position") {
            Kifuwarabe::position(universe, &line);
        } else if 6 < len && &line[starts..7] == "isready" {
            Kifuwarabe::isready();
        } else if 3 < len && &line[starts..4] == "quit" {
            // ループを抜けて終了
            break;
        } else if 15 < len && &line[starts..15] == "setoption name " {
            Kifuwarabe::setoption_name(universe, &line);
        } else if 2 < len && &line[starts..3] == "usi" {
            Kifuwarabe::usi();
        } else if 1 < len && &line[starts..2] == "go" {
            Kifuwarabe::go(universe);
        } else {
            help_chiyuri(&line, len, starts, universe);
        }
    } //loop
}

/// 独自コマンド☆（＾～＾）
fn help_chiyuri(line: &str, len: usize, starts: usize, universe: &mut Universe) {
    // D
    if 2 < len && &line[starts..3] == "do " {
        Chiyuri::do_(universe, line, len, starts);
    // G
    } else if 6 < len && &line[starts..7] == "genmove" {
        Chiyuri::genmove(&universe.game);
    // H
    } else if 7 < len && &line[starts..8] == "how-much" {
        Chiyuri::how_much(line);
    } else if 3 < len && &line[starts..4] == "hash" {
        Chiyuri::hash(universe);
    } else if 3 < len && &line[starts..4] == "kifu" {
        Chiyuri::kifu(universe);
    /* TODO
    } else if 3 < len && &line[starts..4] == "kiki" {
        Chiyuri::kiki(universe);
    */
    // L
    } else if 5 < len && &line[starts..6] == "list40" {
        Chiyuri::list40(universe);
    // P
    } else if 3 < len && &line[starts..4] == "pos0" {
        Chiyuri::pos0(universe);
    } else if 2 < len && &line[starts..3] == "pos" {
        Chiyuri::pos(universe);
    // S
    } else if 7 < len && &line[starts..8] == "startpos" {
        Chiyuri::startpos(universe);
    // R
    } else if 3 < len && &line[starts..4] == "rand" {
        Chiyuri::rand();
    // S
    } else if 3 < len && &line[starts..4] == "same" {
        Chiyuri::same(universe);
    // T
    } else if 3 < len && &line[starts..4] == "teigi::conv" {
        Chiyuri::teigi_conv();
    // U
    } else if 3 < len && &line[starts..4] == "undo" {
        Chiyuri::undo(universe);
    }
}
