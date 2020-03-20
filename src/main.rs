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
pub mod view;

use crate::model::universe::*;
use crate::view::position_view::*;
use crate::view::unit_test::unit_test_view::print_movement_hashset;
use config::*;
use controller::common_use::cu_conv_controller::*;
use controller::main_loop::ml_usi_controller::*;
use controller::movement_generation::mg_controller::*;
use controller::search_part::sp_controller::*;
use controller::unit_test::ut_controller::*;
use model::vo::game_part::gp_square_vo::*;
use model::vo::main_loop::ml_speed_of_light_vo::*;
use model::vo::other_part::op_constants_vo::*;
use model::vo::other_part::op_misc_vo::*;
use rand::Rng;
use std::collections::HashSet;
use std::io;
use view::title_screen::ts_view::*;

fn main() {
    // 光速は定義☆（＾～＾）変化しないから直接アクセスしろだぜ☆（＾～＾）アクセッサは要らないぜ☆（＾～＾）
    let speed_of_light: MLSpeedOfLightVo = MLSpeedOfLightVo::default();
    // 宇宙
    let mut ml_universe_dto: Universe = Universe::default();
    ml_universe_dto.big_bang();

    // [Ctrl]+[C] で強制終了
    loop {
        let mut line: String = if ml_universe_dto.is_empty_command() {
            String::new()
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            ml_universe_dto.pop_command()
        };

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        match io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            Err(e) => panic!("info string Failed to read line. / {}", e),
        };

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!("info string Failed to parse. / {}", e),
        };

        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let starts = 0;

        if len == 0 {
            g_writeln("len==0");
            if !&ml_universe_dto.dialogue_mode {
                // 空打ち１回目なら、対話モードへ☆（＾～＾）
                ml_universe_dto.dialogue_mode = true;
                // タイトル表示
                // １画面は２５行だが、最後の２行は開けておかないと、
                // カーソルが２行分場所を取るんだぜ☆（＾～＾）
                print_title();
            } else {
                // 局面表示
                let s = PositionView::to_string(&ml_universe_dto, &PosNums::Current);
                g_writeln(&s);
            }
        // 文字数の長いものからチェック
        } else if 9 < len && &line[starts..10] == "usinewgame" {
            ml_universe_dto.clear_all_positions();
        } else if line.starts_with("position") {
            // positionコマンドの読取を丸投げ
            controller::main_loop::ml_usi_controller::read_position(
                &line,
                &mut ml_universe_dto,
                &speed_of_light,
            );
        } else if 6 < len && &line[starts..7] == "isready" {
            g_writeln("readyok");
        } else if 3 < len && &line[starts..4] == "quit" {
            // 独自コマンド☆（＾～＾）
            // ループを抜けて終了
            break;
        } else if 2 < len && &line[starts..3] == "usi" {
            g_writeln(&format!("id name {}", ENGINE_NAME));
            g_writeln(&format!("id author {}", ENGINE_AUTHOR));
            g_writeln("usiok");
        } else if 1 < len && &line[starts..2] == "go" {
            ml_universe_dto.get_mut_info().clear();
            // 思考開始と、bestmoveコマンドの返却
            // go btime 40000 wtime 50000 binc 10000 winc 10000
            // let depth = 2;
            let depth = 1;
            let pv = "";
            match get_best_movement(0, depth, 0, &mut ml_universe_dto, &speed_of_light, pv) {
                Some(bestmove) => {
                    // 例： bestmove 7g7f
                    g_writeln(&format!("bestmove {}", bestmove.movement));
                }
                None => {
                    g_writeln("bestmove resign");
                }
            }
        } else {
            parse_extend_command(&line, starts, &mut ml_universe_dto, &speed_of_light);
        }
    } //loop
}

/// 独自コマンド☆（＾～＾）
fn parse_extend_command(
    line: &str,
    mut starts: usize,
    ml_universe_dto: &mut Universe,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // 文字数を調べようぜ☆（＾～＾）
    let len = line.chars().count();
    if line.starts_with("kmugokidir") {
        //}else if 9<len && &line[0..10] == "kmugokidir" {
        g_writeln("9<len kmugokidir");
        // 駒の動きの移動元として有りえる方角
        let piece_type = controller::common_use::cu_random_move_controller::random_piece_type();
        g_writeln(&format!("{}のムーブ元", &piece_type));
        ml_universe_dto.print_kmugoki_dir(*piece_type, speed_of_light);
        g_writeln(""); //改行
    } else if 6 < len && &line[starts..7] == "kmugoki" {
        g_writeln("6<len kmugoki");
        // 駒の動きを出力
        ml_universe_dto.print_kmugoki(&speed_of_light);
    } else if 5 < len && &line[starts..6] == "hirate" {
        // 平手初期局面
        controller::main_loop::ml_usi_controller::read_position(
            &POS_1.to_string(),
            ml_universe_dto,
            &speed_of_light,
        );
    } else if 5 < len && &line[starts..6] == "kikisu" {
        // 利き数表示
        controller::main_loop::ml_main_controller::cmd_kikisu(&ml_universe_dto, &speed_of_light);
    } else if 5 < len && &line[starts..6] == "random_piece_type" {
        g_writeln("5<len random_piece_type");
        // 乱駒種類
        let piece_type = controller::common_use::cu_random_move_controller::random_piece_type();
        g_writeln(&format!("乱駒種類={}", &piece_type));
    } else if 5 < len && &line[starts..6] == "sasite" {
        // FIXME 合法手とは限らない
        let mut ss_potential_hashset = HashSet::<u64>::new();
        get_up_potential_movement(
            &ml_universe_dto.get_position(),
            &speed_of_light,
            &mut |movement_hash| {
                ss_potential_hashset.insert(movement_hash);
            },
        );
        g_writeln("----指し手生成 ここから----");
        print_movement_hashset(&ss_potential_hashset);
        g_writeln("----指し手生成 ここまで----");
    } else if 4 < len && &line[starts..5] == "random_ms" {
        // 乱升
        let sq = controller::common_use::cu_random_move_controller::random_square();
        g_writeln(&format!("乱升={}", sq.to_usquare()));
    } else if 3 < len && &line[starts..4] == "teigi::conv" {
        g_writeln("teigi::convのテスト");

        for ms in 11..19 {
            for hash in 0..10 {
                let sq = Square::from_usquare(ms);
                let next = push_sq_to_hash(hash, &sq);
                let (hash_orig, sq_orig) = pop_sq_from_hash(next);
                g_writeln( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,sq_orig.to_usquare()
                ));
            }
        }
    } else if 3 < len && &line[starts..4] == "hash" {
        g_writeln("局面ハッシュ表示");
        let s = ml_universe_dto.get_all_position_hash_text();
        g_writeln(&s);
    } else if 3 < len && &line[starts..4] == "kifu" {
        g_writeln("棋譜表示");
        let s = ml_universe_dto.get_position().get_moves_history_text();
        g_writeln(&s);
    } else if 3 < len && &line[starts..4] == "rand" {
        g_writeln("3<len rand");
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        g_writeln(&format!("乱数={}", secret_number));
    } else if 3 < len && &line[starts..4] == "same" {
        let count = ml_universe_dto.count_same_ky();
        g_writeln(&format!("同一局面調べ count={}", count));
    } else if 3 < len && &line[starts..4] == "undo" {
        if !ml_universe_dto.undo_move(&speed_of_light) {
            g_writeln(&format!(
                "ply={} を、これより戻せません",
                ml_universe_dto.get_position().get_ply()
            ));
        }
    } else if 8 < len && &line[starts..9] == "unit-test" {
        starts += 4;
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0 < (len - starts) && &line[starts..=starts] == " " {
            starts += 1;
        }
        // いろいろな動作テスト
        g_writeln(&format!("unit-test starts={} len={}", starts, len));
        unit_test(&line, &mut starts, len, ml_universe_dto, &speed_of_light);
    //g_writeln( &ml_universe_dto.pop_command() );
    } else if 2 < len && &line[starts..3] == "do " {
        starts += 3;
        // コマンド読取。棋譜に追加され、手目も増える
        if read_sasite(&line, &mut starts, len, ml_universe_dto) {
            // 手目を戻す
            ml_universe_dto.get_position_mut().add_ply(-1);
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = ml_universe_dto.get_position().get_ply();
            let ss = ml_universe_dto.get_position().get_moves_history()[ply as usize].clone();
            ml_universe_dto.do_move(&ss, speed_of_light);
        }
    } else if 3 < len && &line[starts..4] == "pos0" {
        // 初期局面表示
        let s = PositionView::to_string(&ml_universe_dto, &PosNums::Start);
        g_writeln(&s);
    } else if 2 < len && &line[starts..3] == "pos" {
        // 現局面表示
        let s = PositionView::to_string(&ml_universe_dto, &PosNums::Current);
        g_writeln(&s);
    }
}
