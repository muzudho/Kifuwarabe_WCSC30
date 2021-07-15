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
extern crate serde;
extern crate toml;

// Rust言語の mod や ソース置き場の説明
//     「Rust のモジュールシステム」
//      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
//
// 使いたい ディレクトリー名を pub mod しろだぜ☆（＾～＾）
// 別のアプリにも見えるようにしたけりゃ pub mod にしろだぜ☆（＾～＾）
mod config;
mod entities;
mod movegen;
mod position;
mod record;
mod search;
mod take1base;
mod usi;
mod view;

use crate::entities::cosmic::universe::Universe;
use crate::usi::main_loop;
use crate::usi::yumemi::Yumemi;

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
