use crate::config::*;
use crate::cosmic::daydream::Tree;
use crate::cosmic::shogi::playing::{Game, PosNums};
use crate::cosmic::smart::square::AbsoluteAddress;
use crate::cosmic::universe::Universe;
use crate::law::cryptographic::*;
use crate::law::generate_move::movement_generator::*;
use crate::law::speed_of_light::*;
use crate::law::usi::*;
use crate::spaceship::equipment::Telescope;
use crate::white_hole::io::IO;
use crate::white_hole::visual::game_view::GameView;
use crate::white_hole::visual::title_screen::ts_view::print_title;
use crate::white_hole::visual::unit_test::unit_test_view::print_movement_hashset;
use rand::Rng;
use std::collections::HashSet;
use std::io as std_io;

/// 船長：きふわらべ
///
/// 対局で許されている命令だけをするぜ☆（＾～＾）
pub struct Kifuwarabe {}
impl Kifuwarabe {
    pub fn catch_the_message(universe: &mut Universe) -> (String, usize, usize) {
        let mut line: String = if universe.is_empty_command() {
            String::new()
        } else {
            // バッファーに溜まっていれば☆（＾～＾）
            universe.pop_command()
        };

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        match std_io::stdin().read_line(&mut line) {
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

        (line, len, starts)
    }
    /// bestmoveコマンドを送るぜ☆（＾～＾） 思考するのもこの中だぜ☆（＾～＾）
    pub fn go(speed_of_light: &SpeedOfLight, universe: &mut Universe) {
        // go btime 40000 wtime 50000 binc 10000 winc 10000
        let ts = Tree::first_move(speed_of_light, universe);
        // その手を選んだ理由☆（＾～＾）
        universe.game.info.print(
            0,
            ts.get_sum_state(),
            ts.get_value(),
            ts.get_king_catch(),
            ts.get_movement_hash(),
            &ts.reason,
        );
        // 例: bestmove 7g7f
        // 例: bestmove resign
        IO::writeln(&format!("bestmove {}", ts.to_movement()));
    }
    pub fn isready() {
        IO::writeln("readyok");
    }
    pub fn position(speed_of_light: &SpeedOfLight, universe: &mut Universe, line: &String) {
        // positionコマンドの読取を丸投げ
        set_position(&line, universe, &speed_of_light);
    }
    pub fn setoption_name(universe: &mut Universe, line: &String) {
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
    }
    pub fn usi() {
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
        IO::writeln("option name MaxDepth type spin default 1 min 1 max 5");
        IO::writeln("usiok");
    }
    pub fn usinewgame(universe: &mut Universe) {
        universe.game.clear_all_positions();
    }
}

/// 副船長：ちゆり
///
/// 対局でやっちゃいかん命令なら任せろだぜ☆（＾～＾）
pub struct Chiyuri {}
impl Chiyuri {
    pub fn do_(
        speed_of_light: &SpeedOfLight,
        universe: &mut Universe,
        line: &str,
        len: usize,
        mut starts: usize,
    ) {
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
    }
    pub fn genmove(speed_of_light: &SpeedOfLight, game: &Game) {
        // Generation move.
        // FIXME 合法手とは限らない
        let mut ss_potential_hashset = HashSet::<u64>::new();
        get_potential_movement(&game, &speed_of_light, &mut |movement_hash| {
            ss_potential_hashset.insert(movement_hash);
        });
        IO::writeln("----指し手生成(合法手とは限らない) ここから----");
        print_movement_hashset(&ss_potential_hashset);
        IO::writeln("----指し手生成(合法手とは限らない) ここまで----");
    }
    pub fn hash(universe: &Universe) {
        IO::writeln("局面ハッシュ表示");
        let s = universe.game.get_all_position_hash_text();
        IO::writeln(&s);
    }
    pub fn how_much(line: &str) {
        // Example: how-much 7g7f
        let bestmove = &line[9..];
        IO::writeln(&format!("Debug   | bestmove=|{}|", bestmove));
    }
    pub fn kifu(universe: &Universe) {
        IO::writeln("棋譜表示");
        let s = universe.game.get_moves_history_text();
        IO::writeln(&s);
    }
    pub fn len0(universe: &mut Universe) {
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
    }
    pub fn pos(universe: &Universe) {
        // 現局面表示
        let s = GameView::to_string(&universe.game, &PosNums::Current);
        IO::writeln(&s);
    }
    pub fn pos0(universe: &Universe) {
        // 初期局面表示
        let s = GameView::to_string(&universe.game, &PosNums::Start);
        IO::writeln(&s);
    }
    pub fn rand() {
        IO::writeln("3<len rand");
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        IO::writeln(&format!("乱数={}", secret_number));
    }
    pub fn same(universe: &Universe) {
        let count = universe.game.count_same_ky();
        IO::writeln(&format!("同一局面調べ count={}", count));
    }
    pub fn startpos(speed_of_light: &SpeedOfLight, universe: &mut Universe) {
        // 平手初期局面
        set_position(&POS_1.to_string(), universe, &speed_of_light);
    }
    pub fn teigi_conv() {
        IO::writeln("teigi::convのテスト");

        for ms in 11..19 {
            for hash in 0..10 {
                let sq = AbsoluteAddress::from_address(ms);
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
    }
    pub fn undo(speed_of_light: &SpeedOfLight, universe: &mut Universe) {
        if !universe.game.undo_move(&speed_of_light) {
            IO::writeln(&format!(
                "ply={} を、これより戻せません",
                universe.game.history.ply
            ));
        }
    }
}

/// 乗組員：夢美
pub struct Yumemi {}
impl Yumemi {
    /// 望遠鏡を覗き込みましょう。
    pub fn look_into_the_telescope() {
        Telescope::look();
    }
}
