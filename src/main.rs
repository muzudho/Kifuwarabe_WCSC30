//!
//! きふわらべＷＣＳＣ３０
//!
extern crate rand;
#[macro_use]
extern crate lazy_static;

/**
 * Rust言語の mod や ソース置き場の説明
 *      「Rust のモジュールシステム」
 *      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
 */
// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる
pub mod config;
pub mod controller;
pub mod model;

use config::*;
use controller::common::conv::*;
use controller::communication::usi::*;
use controller::consoles::unit_test::*;
use controller::consoles::visuals::dumps::*;
use controller::consoles::visuals::title::*;
use controller::movement_generation::mg_main::*;
use controller::piece_movetrics::pm_main::*;
use model::master::constants::*;
use model::master::misc::*;
use model::master::square::*;
use model::universe::*;
use model::vo::speed_of_light::*;
use rand::Rng;
use std::collections::HashSet;
use std::io;

fn main() {
    // 光速は定義☆（＾～＾）変化しないから直接アクセスしろだぜ☆（＾～＾）アクセッサは要らないぜ☆（＾～＾）
    let speed_of_light: SpeedOfLight = SpeedOfLight::new();
    // 宇宙
    let mut universe: Universe = Universe::new();
    universe.big_bang();

    // [Ctrl]+[C] で強制終了
    loop {
        let mut line: String;
        if universe.is_empty_command() {
            line = String::new();
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            line = universe.pop_command();
            //g_writeln( &line );
        }

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        io::stdin()
            .read_line(&mut line)
            .ok() // read_lineの返り値オブジェクトResult の okメソッド
            .expect("info Failed to read line"); // OKで無かった場合のエラーメッセージ

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line: String = line.trim().parse().ok().expect("info Failed to parse");

        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let mut starts = 0;

        if len == 0 {
            g_writeln("len==0");
            if !&universe.get_dialogue_part().dialogue_mode {
                // 空打ち１回目なら、対話モードへ☆（＾～＾）
                universe.get_dialogue_part_mut().dialogue_mode = true;
                // タイトル表示
                // １画面は２５行だが、最後の２行は開けておかないと、
                // カーソルが２行分場所を取るんだぜ☆（＾～＾）
                hyoji_title();
            } else {
                // 局面表示
                let s = &universe.kaku_ky(&KyNums::Current);
                g_writeln(&s);
            }
        // 文字数の長いものからチェック
        } else if line.starts_with("kmugokidir") {
            //}else if 9<len && &line[0..10] == "kmugokidir" {
            g_writeln("9<len kmugokidir");
            // 駒の動きの移動元として有りえる方角
            let kms = controller::thinking::randommove::rnd_kms();
            g_writeln(&format!("{}のムーブ元", &kms));
            universe.hyoji_kmugoki_dir(kms);
            g_writeln(""); //改行
        } else if 9 < len && &line[starts..10] == "usinewgame" {
            universe.clear_all_positions();
        } else if line.starts_with("position") {
            // positionコマンドの読取を丸投げ
            controller::communication::usi::read_position(&line, &mut universe, &speed_of_light);
        } else if 6 < len && &line[starts..7] == "isready" {
            g_writeln("readyok");
        } else if 6 < len && &line[starts..7] == "kmugoki" {
            g_writeln("6<len kmugoki");
            // 駒の動きを出力
            universe.hyoji_kmugoki();
        } else if 5 < len && &line[starts..6] == "hirate" {
            // 平手初期局面
            controller::communication::usi::read_position(
                &KY1.to_string(),
                &mut universe,
                &speed_of_light,
            );
        } else if 5 < len && &line[starts..6] == "kikisu" {
            // 利き数表示
            controller::consoles::commands::cmd_kikisu(&universe, &speed_of_light);
        } else if 5 < len && &line[starts..6] == "rndkms" {
            g_writeln("5<len rndkms");
            // 乱駒種類
            let kms = controller::thinking::randommove::rnd_kms();
            g_writeln(&format!("乱駒種類={}", &kms));
        } else if 5 < len && &line[starts..6] == "sasite" {
            // FIXME 合法手とは限らない
            let mut ss_potential_hashset = HashSet::<u64>::new();
            get_potential_movement(
                &universe.get_search_part(),
                &speed_of_light,
                |movement_hash| {
                    ss_potential_hashset.insert(movement_hash);
                },
            );
            g_writeln("----指し手生成 ここから----");
            hyoji_ss_hashset(&ss_potential_hashset);
            g_writeln("----指し手生成 ここまで----");
        } else if 4 < len && &line[starts..5] == "rndms" {
            // 乱升
            let sq = controller::thinking::randommove::random_square();
            g_writeln(&format!("乱升={}", sq.to_umasu()));
        } else if 3 < len && &line[starts..4] == "teigi::conv" {
            g_writeln("teigi::convのテスト");

            for ms in 11..19 {
                for hash in 0..10 {
                    let sq = Square::from_umasu(ms);
                    let next = push_sq_to_hash(hash, &sq);
                    let (hash_orig, sq_orig) = pop_sq_from_hash(next);
                    g_writeln( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                        ,hash
                        ,ms
                        ,next
                        ,hash_orig
                        ,sq_orig.to_umasu()
                    ));
                }
            }
        } else if 3 < len && &line[starts..4] == "hash" {
            g_writeln("局面ハッシュ表示");
            let s = universe.get_all_position_hash_text();
            g_writeln(&s);
        } else if 3 < len && &line[starts..4] == "kifu" {
            g_writeln("棋譜表示");
            let s = universe.get_search_part().get_moves_history_text();
            g_writeln(&s);
        } else if 3 < len && &line[starts..4] == "quit" {
            // ループを抜けて終了
            break;
        } else if 3 < len && &line[starts..4] == "rand" {
            g_writeln("3<len rand");
            // 乱数の試し
            let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
            g_writeln(&format!("乱数={}", secret_number));
        } else if 3 < len && &line[starts..4] == "same" {
            let count = universe.count_same_ky();
            g_writeln(&format!("同一局面調べ count={}", count));
        } else if 3 < len && &line[starts..4] == "test" {
            starts += 4;
            // 続きにスペース「 」が１つあれば読み飛ばす
            if 0 < (len - starts) && &line[starts..(starts + 1)] == " " {
                starts += 1;
            }
            // いろいろな動作テスト
            g_writeln(&format!("test starts={} len={}", starts, len));
            test(&line, &mut starts, len, &mut universe, &speed_of_light);
        //g_writeln( &universe.pop_command() );
        } else if 3 < len && &line[starts..4] == "undo" {
            if !universe.undo_ss(&speed_of_light) {
                g_writeln(&format!(
                    "ply={} を、これより戻せません",
                    universe.get_search_part().get_ply()
                ));
            }
        } else if 2 < len && &line[starts..3] == "do " {
            starts += 3;
            // コマンド読取。棋譜に追加され、手目も増える
            if read_sasite(&line, &mut starts, len, &mut universe) {
                // 手目を戻す
                universe.get_search_part_mut().add_ply(-1);
                // 入っている指し手の通り指すぜ☆（＾～＾）
                let ply = universe.get_search_part().get_ply();
                let ss = universe.get_search_part().get_moves_history()[ply as usize].clone();
                universe.do_ss(&ss, &speed_of_light);
            }
        } else if 2 < len && &line[starts..3] == "pos0" {
            // 初期局面表示
            let s = universe.kaku_ky(&KyNums::Start);
            g_writeln(&s);
        } else if 2 < len && &line[starts..3] == "usi" {
            g_writeln(&format!("id name {}", ENGINE_NAME));
            g_writeln(&format!("id author {}", ENGINE_AUTHOR));
            g_writeln("usiok");
        } else if 1 < len && &line[starts..2] == "go" {
            // 思考開始と、bestmoveコマンドの返却
            // go btime 40000 wtime 50000 binc 10000 winc 10000
            let bestmove = think(&mut universe, &speed_of_light);
            // 例： bestmove 7g7f
            g_writeln(&format!("bestmove {}", bestmove));
        } else if 1 < len && &line[starts..2] == "pos" {
            // 現局面表示
            let s = &universe.kaku_ky(&KyNums::Current);
            g_writeln(&s);
        }
    } //loop
}
