use crate::entities::spaceship::equipment::Beam;
use crate::Universe;

mod chiyuri;
mod kifuwarabe;
pub mod yumemi;
use std::io as std_io;

/// 副船長：ちゆり
///
/// 対局でやっちゃいかん命令なら任せろだぜ☆（＾～＾）
pub struct Chiyuri {}

/// 船長：きふわらべ
///
/// 対局で許されている命令だけをするぜ☆（＾～＾）
pub struct Kifuwarabe {}

/// 乗組員：夢美
pub struct Yumemi {}

pub fn main_loop(universe: &mut Universe) {
    loop {
        let mut input: String = String::new();

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        match std_io::stdin().read_line(&mut input) {
            Ok(_n) => {}
            Err(e) => std::panic::panic_any(Beam::trouble(&format!(
                "(Err.28)  Failed to read line. / {}",
                e
            ))),
        };

        let tokens: Vec<&str> = input.split(' ').collect();

        if tokens.len() == 0 {
            // 任せろだぜ☆（＾～＾）
            Chiyuri::len0(universe);
        // 文字数の長いものからチェック
        } else {
            match tokens[0] {
                "usinewgame" => {
                    Kifuwarabe::usinewgame(universe);
                }
                "position" => {
                    if tokens[3] == "*0" {
                        // 将棋所の連続対局中に
                        // 相手が 時間切れを知らずに bestmove を返すと、
                        // 将棋所は `isready` など次の対局が始まっている最中に
                        // `position startpos moves *0` を返してくる。
                        // この `*0` をパースできずに落ちることがあるので、無視するぜ（＾～＾）
                        continue;
                    }
                    Kifuwarabe::position(universe, &tokens);
                }
                "isready" => {
                    Kifuwarabe::isready();
                }
                "quit" => {
                    // ループを抜けて終了
                    break;
                }
                "setoption" => {
                    if tokens[1] == "name" {
                        Kifuwarabe::setoption_name(universe, &tokens);
                    }
                }
                "usi" => {
                    Kifuwarabe::usi();
                }
                "go" => {
                    Kifuwarabe::go(universe);
                }
                "gameover" => {
                    // gameover win
                    // gameover lose
                    // gameover draw
                    // 時間切れのときなど、将棋所から このメッセージがくるぜ（＾～＾）
                    // TODO 時間切れと知らず指し手を返すと 将棋所の連続対局で不具合を起こすから、指し手は返すなだぜ（＾～＾）
                    // といっても、そんなことは難しい（＾～＾）
                }
                _ => {
                    help_chiyuri(universe, &tokens);
                }
            }
        }
    } //loop
}

/// 独自コマンド☆（＾～＾）
fn help_chiyuri(universe: &mut Universe, tokens: &Vec<&str>) {
    match tokens[0] {
        "do" => {
            // do 7g7f
            Chiyuri::do_(universe, tokens[1]);
        }
        "genmove" => {
            Chiyuri::genmove(&universe.game);
        }
        "how-much" => {
            Chiyuri::how_much(tokens);
        }
        "hash" => {
            Chiyuri::hash(universe);
        }
        "record" => {
            // 棋譜（指し手）の表示
            Chiyuri::record(universe);
            // L
        }
        // "kiki" => {
        //     Chiyuri::kiki(universe);
        // }
        "list40" => {
            Chiyuri::list40(universe);
        }
        "pos0" => {
            Chiyuri::pos0(universe);
        }
        "pos" => {
            Chiyuri::pos(universe);
        }
        "startpos" => {
            Chiyuri::startpos(universe);
        }
        "rand" => {
            Chiyuri::rand();
        }
        "same" => {
            Chiyuri::same(universe);
        }
        "teigi::conv" => {
            Chiyuri::teigi_conv();
        }
        "undo" => {
            Chiyuri::undo(universe);
        }
        _ => {}
    }
}
