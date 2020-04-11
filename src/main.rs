//!
//! きふわらべＷＣＳＣ３０
//!
extern crate rand;
#[macro_use]
extern crate lazy_static;

// Rust言語の mod や ソース置き場の説明
//     「Rust のモジュールシステム」
//      https://qiita.com/skitaoka/items/753a519d720a1ccebb0d
//
// use したい モジュールは、最初に読み取られる　この main.rs ファイルに並べる
pub mod config;
pub mod controller;
pub mod model;
pub mod view;

use crate::model::univ::gam::misc::square::*;
use crate::model::univ::game::*;
use crate::model::univ::speed_of_light::*;
use crate::model::univ::usi::*;
use crate::model::universe::*;
use crate::view::game_view::*;
use crate::view::unit_test::unit_test_view::print_movement_hashset;
use config::*;
use controller::common_use::cu_conv_controller::*;
use controller::io::*;
use controller::main_loop::ml_usi_controller::*;
use controller::movement_generation::mg_controller::*;
use controller::searching::tree::*;
use controller::unit_test::ut_controller::*;
use rand::Rng;
use std::collections::HashSet;
use std::io;
use view::title_screen::ts_view::*;

fn main() {
    // 光速は定義☆（＾～＾）変化しないから直接アクセスしろだぜ☆（＾～＾）アクセッサは要らないぜ☆（＾～＾）
    let speed_of_light: MLSpeedOfLightVo = MLSpeedOfLightVo::default();
    // 宇宙
    let mut universe: Universe = Universe::default();
    universe.big_bang();

    // テスト
    test_rotation();

    // [Ctrl]+[C] で強制終了
    loop {
        let mut line: String = if universe.is_empty_command() {
            String::new()
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            universe.pop_command()
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
            IO::writeln("len==0");
            if !&universe.dialogue_mode {
                // 空打ち１回目なら、対話モードへ☆（＾～＾）
                universe.dialogue_mode = true;
                // タイトル表示
                // １画面は２５行だが、最後の２行は開けておかないと、
                // カーソルが２行分場所を取るんだぜ☆（＾～＾）
                print_title();
            } else {
                // 局面表示
                let s = GameView::to_string(&universe.game, &PosNums::Current);
                IO::writeln(&s);
            }
        // 文字数の長いものからチェック
        } else if 9 < len && &line[starts..10] == "usinewgame" {
            universe.game.clear_all_positions();
        } else if line.starts_with("position") {
            // positionコマンドの読取を丸投げ
            controller::main_loop::ml_usi_controller::read_position(
                &line,
                &mut universe,
                &speed_of_light,
            );
        } else if 6 < len && &line[starts..7] == "isready" {
            IO::writeln("readyok");
        } else if 3 < len && &line[starts..4] == "quit" {
            // 独自コマンド☆（＾～＾）
            // ループを抜けて終了
            break;
        } else if 15 < len && &line[starts..15] == "setoption name " {
            // Example: setoption name USI_Ponder value true
            if let Some(x) = line[15..].find(' ') {
                let name = &line[15..(x + 15)];
                // IO::writeln(&format!("Debug name=|{}|", name));
                let value = &line[(x + 22)..];
                // IO::writeln(&format!("Debug value=|{}|", value));
                if name == "MaxDepth" {
                    universe.option_max_depth = value.parse().unwrap();
                }
            };
        } else if 2 < len && &line[starts..3] == "usi" {
            IO::writeln(&format!("id name {}", ENGINE_NAME));
            IO::writeln(&format!("id author {}", ENGINE_AUTHOR));
            /*
            IO::writeln("option name BookFile type string default public.bin");
            IO::writeln("option name UseBook type check default true");
            IO::writeln("option name Selectivity type spin default 2 min 0 max 4");
            IO::writeln(
                "option name Style type combo default Normal var Solid var Normal var Risky",
            );
            IO::writeln("option name ResetLearning type button");
            IO::writeln("option name LearningFile type filename default <empty>");
            */
            IO::writeln("option name MaxDepth type spin default 1 min 1 max 3");
            IO::writeln("usiok");
        } else if 1 < len && &line[starts..2] == "go" {
            universe.game.info.clear();
            // 思考開始と、bestmoveコマンドの返却
            // go btime 40000 wtime 50000 binc 10000 winc 10000
            let pv = "";
            match get_best_movement(
                0,
                universe.option_max_depth - 1,
                0,
                &mut universe.game,
                &speed_of_light,
                pv,
            ) {
                Some(bestmove) => {
                    // 例： bestmove 7g7f
                    IO::writeln(&format!("bestmove {}", bestmove.movement));
                }
                None => {
                    IO::writeln("bestmove resign");
                }
            }
        } else {
            parse_extend_command(&line, starts, &mut universe, &speed_of_light);
        }
    } //loop
}

/// 独自コマンド☆（＾～＾）
fn parse_extend_command(
    line: &str,
    mut starts: usize,
    universe: &mut Universe,
    speed_of_light: &MLSpeedOfLightVo,
) {
    // 文字数を調べようぜ☆（＾～＾）
    let len = line.chars().count();
    // C
    if 6 < len && &line[starts..7] == "contnum" {
        // 利き数表示
        controller::main_loop::ml_main_controller::cmd_kikisu(&universe, &speed_of_light);
    // D
    } else if 2 < len && &line[starts..3] == "do " {
        starts += 3;
        // コマンド読取。棋譜に追加され、手目も増える
        if read_sasite(&line, &mut starts, len, universe) {
            // 手目を戻す
            universe.game.history.ply -= 1;
            // 入っている指し手の通り指すぜ☆（＾～＾）
            let ply = universe.game.history.ply;
            let ss = universe.game.history.movements[ply as usize].clone();
            universe.game.do_move(&ss, speed_of_light);
        }
    // G
    } else if 6 < len && &line[starts..7] == "genmove" {
        // Generation move.
        // FIXME 合法手とは限らない
        let mut ss_potential_hashset = HashSet::<u64>::new();
        get_up_potential_movement(&universe.game, &speed_of_light, &mut |movement_hash| {
            ss_potential_hashset.insert(movement_hash);
        });
        IO::writeln("----指し手生成(合法手とは限らない) ここから----");
        print_movement_hashset(&ss_potential_hashset);
        IO::writeln("----指し手生成(合法手とは限らない) ここまで----");
    // H
    } else if 7 < len && &line[starts..8] == "how-much" {
        // Example: how-much 7g7f
        let bestmove = &line[9..];
        IO::writeln(&format!("Debug   | bestmove=|{}|", bestmove));
    } else if 3 < len && &line[starts..4] == "hash" {
        IO::writeln("局面ハッシュ表示");
        let s = universe.game.get_all_position_hash_text();
        IO::writeln(&s);
    // K
    } else if line.starts_with("kmugokidir") {
        //}else if 9<len && &line[0..10] == "kmugokidir" {
        IO::writeln("9<len kmugokidir");
        // 駒の動きの移動元として有りえる方角
        let piece_type = controller::common_use::cu_random_move_controller::random_piece_type();
        IO::writeln(&format!("{}のムーブ元", &piece_type));
        universe.print_kmugoki_dir(*piece_type, speed_of_light);
        IO::writeln(""); //改行
    } else if 6 < len && &line[starts..7] == "kmugoki" {
        IO::writeln("6<len kmugoki");
        // 駒の動きを出力
        universe.print_kmugoki(&speed_of_light);
    } else if 3 < len && &line[starts..4] == "kifu" {
        IO::writeln("棋譜表示");
        let s = universe.game.get_moves_history_text();
        IO::writeln(&s);
    // P
    } else if 3 < len && &line[starts..4] == "pos0" {
        // 初期局面表示
        let s = GameView::to_string(&universe.game, &PosNums::Start);
        IO::writeln(&s);
    } else if 2 < len && &line[starts..3] == "pos" {
        // 現局面表示
        let s = GameView::to_string(&universe.game, &PosNums::Current);
        IO::writeln(&s);
    // S
    } else if 7 < len && &line[starts..8] == "startpos" {
        // 平手初期局面
        controller::main_loop::ml_usi_controller::read_position(
            &POS_1.to_string(),
            universe,
            &speed_of_light,
        );
    // R
    } else if 5 < len && &line[starts..6] == "random_piece_type" {
        IO::writeln("5<len random_piece_type");
        // 乱駒種類
        let piece_type = controller::common_use::cu_random_move_controller::random_piece_type();
        IO::writeln(&format!("乱駒種類={}", &piece_type));
    } else if 4 < len && &line[starts..5] == "random_ms" {
        // 乱升
        let square = controller::common_use::cu_random_move_controller::random_square();
        IO::writeln(&format!("乱升={}", square.address));
    } else if 3 < len && &line[starts..4] == "rand" {
        IO::writeln("3<len rand");
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        IO::writeln(&format!("乱数={}", secret_number));
    // S
    } else if 3 < len && &line[starts..4] == "same" {
        let count = universe.game.count_same_ky();
        IO::writeln(&format!("同一局面調べ count={}", count));
    // T
    } else if 3 < len && &line[starts..4] == "teigi::conv" {
        IO::writeln("teigi::convのテスト");

        for ms in 11..19 {
            for hash in 0..10 {
                let sq = Square::from_address(ms);
                let next = push_sq_to_hash(hash, &sq);
                let (hash_orig, square_orig) = pop_sq_from_hash(next);
                IO::writeln( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,square_orig.address
                ));
            }
        }
    // U
    } else if 3 < len && &line[starts..4] == "undo" {
        if !universe.game.undo_move(&speed_of_light) {
            IO::writeln(&format!(
                "ply={} を、これより戻せません",
                universe.game.history.ply
            ));
        }
    } else if 8 < len && &line[starts..9] == "unit-test" {
        starts += 4;
        // 続きにスペース「 」が１つあれば読み飛ばす
        if 0 < (len - starts) && &line[starts..=starts] == " " {
            starts += 1;
        }
        // いろいろな動作テスト
        IO::writeln(&format!("unit-test starts={} len={}", starts, len));
        unit_test(&line, &mut starts, len, universe, &speed_of_light);
        //IO::writeln( &ml_universe_dto.pop_command() );
    }
}

fn test_dort(test_name: &str, expected: &str, actual: &DictOrthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_d45ort(test_name: &str, expected: &str, actual: &Degree45Orthant) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}
fn test_rsq(test_name: &str, expected: &str, actual: &RelativeSquare) {
    debug_assert!(
        format!("{:?}", actual) == expected,
        format!("{}: expected={} | actual={:?}", test_name, expected, actual)
    );
}

fn test_rotation() {
    // 辞書象限のテスト
    {
        let mut ort = DictOrthant::from_file_and_rank(0, -1);
        test_dort("e1", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, -1);
        test_dort("e2", "IV", &ort);
        ort = DictOrthant::from_file_and_rank(1, 0);
        test_dort("e3", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(1, 1);
        test_dort("e4", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(0, 1);
        test_dort("e5", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 1);
        test_dort("e6", "II", &ort);
        ort = DictOrthant::from_file_and_rank(-1, 0);
        test_dort("e7", "IOrIII", &ort);
        ort = DictOrthant::from_file_and_rank(-1, -1);
        test_dort("e8", "IOrIII", &ort);
    }
    // 45°回転象限のテスト
    {
        let mut ort = Degree45Orthant::from_file_and_rank(0, -1);
        test_d45ort("f1", "CoIIIOrCoIV", &ort);
        ort = Degree45Orthant::from_file_and_rank(1, -1);
        test_d45ort("f2", "IVOrI", &ort);
        ort = Degree45Orthant::from_file_and_rank(1, 0);
        test_d45ort("f3", "IVOrI", &ort);
        ort = Degree45Orthant::from_file_and_rank(1, 1);
        test_d45ort("f4", "IVOrI", &ort);
        ort = Degree45Orthant::from_file_and_rank(0, 1);
        test_d45ort("f5", "CoIOrCoII", &ort);
        ort = Degree45Orthant::from_file_and_rank(-1, 1);
        test_d45ort("f6", "IIOrIII", &ort);
        ort = Degree45Orthant::from_file_and_rank(-1, 0);
        test_d45ort("f7", "IIOrIII", &ort);
        ort = Degree45Orthant::from_file_and_rank(-1, -1);
        test_d45ort("f8", "IIOrIII", &ort);
    }
    // 相対番地のテスト
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("b1", "(0x -1y -1adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(1, -1);
        test_rsq("b2", "(1x -1y 9adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(1, 0);
        test_rsq("b3", "(1x 0y 10adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(1, 1);
        test_rsq("b4", "(1x 1y 11adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("b5", "(0x 1y 1adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(-1, 1);
        test_rsq("b6", "(-1x 1y -9adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(-1, 0);
        test_rsq("b7", "(-1x 0y -10adr)", &rsq);
        rsq = RelativeSquare::from_file_and_rank(-1, -1);
        test_rsq("b8", "(-1x -1y -11adr)", &rsq);
    }
    // 45°回転のテスト
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("a1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a2", "(1x -1y 9adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a3", "(1x 0y 10adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a4", "(1x 1y 11adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a5", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a6", "(-1x 1y -9adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a7", "(-1x 0y -10adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a8", "(-1x -1y -11adr)", &rsq);
        rsq = rsq.rotate_45_countercrockwise();
        test_rsq("a9", "(0x -1y -1adr)", &rsq);
    }
    // 90°回転のテスト＜その１＞
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("c1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c2", "(1x 0y 10adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c3", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c4", "(-1x 0y -10adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("c5", "(0x -1y -1adr)", &rsq);
    }
    // 90°回転のテスト＜その２＞
    {
        let mut rsq = RelativeSquare::from_file_and_rank(1, -1);
        test_rsq("d1", "(1x -1y 9adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d2", "(1x 1y 11adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d3", "(-1x 1y -9adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d4", "(-1x -1y -11adr)", &rsq);
        rsq = rsq.rotate_90_countercrockwise();
        test_rsq("d5", "(1x -1y 9adr)", &rsq);
    }
    // 桂馬のテスト
    {
        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("g1", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate_rel(&Rotation::Ccw45);
        test_rsq("g2", "(1x -1y 9adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g3", "(1x -2y 8adr)", &rsq);

        let mut rsq = RelativeSquare::from_file_and_rank(0, -1);
        test_rsq("g4", "(0x -1y -1adr)", &rsq);
        rsq = rsq.rotate_rel(&Rotation::Ccw315);
        test_rsq("g5", "(-1x -1y -11adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g6", "(-1x -2y -12adr)", &rsq);

        let mut rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("g7", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate_rel(&Rotation::Ccw45);
        test_rsq("g8", "(-1x 1y -9adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g9", "(-1x 2y -8adr)", &rsq);

        let mut rsq = RelativeSquare::from_file_and_rank(0, 1);
        test_rsq("g10", "(0x 1y 1adr)", &rsq);
        rsq = rsq.rotate_rel(&Rotation::Ccw315);
        test_rsq("g11", "(1x 1y 11adr)", &rsq);
        rsq = rsq.double_rank();
        test_rsq("g12", "(1x 2y 12adr)", &rsq);
    }
}
