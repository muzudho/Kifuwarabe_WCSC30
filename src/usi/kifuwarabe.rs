use crate::config::*;
use crate::entities::cosmic::universe::Universe;
use crate::entities::law::usi::*;
use crate::entities::spaceship::equipment::Beam;
use crate::position::to_move_code;
use crate::search::Tree;
use crate::usi::Kifuwarabe;
use crate::view::print_info;
use std::io as std_io;

impl Kifuwarabe {
    pub fn catch_the_message() -> (String, usize, usize) {
        let mut line: String = String::new();

        // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
        match std_io::stdin().read_line(&mut line) {
            Ok(_n) => {}
            Err(e) => std::panic::panic_any(Beam::trouble(&format!(
                "(Err.28)  Failed to read line. / {}",
                e
            ))),
        };

        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => std::panic::panic_any(Beam::trouble(&format!(
                "(Err.38)  Failed to parse. / {}",
                e
            ))),
        };

        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let starts = 0;

        (line, len, starts)
    }
    /// bestmoveコマンドを送るぜ☆（＾～＾） 思考するのもこの中だぜ☆（＾～＾）
    pub fn go(universe: &mut Universe) {
        // go btime 40000 wtime 50000 binc 10000 winc 10000
        let mut tree = Tree::new(universe.option_depth_not_to_give_up);
        let (node_value, bestmove) = tree.iteration_deeping(universe);
        // その手を選んだ理由☆（＾～＾）
        print_info(
            &mut universe.game.info,
            None,
            Some((tree.state_nodes, tree.nps())),
            Some(node_value),
            Some(bestmove),
            &None,
        );
        // 例: bestmove 7g7f
        // 例: bestmove resign
        Beam::shoot(&format!("bestmove {}", to_move_code(bestmove)));
    }
    pub fn isready() {
        Beam::shoot("readyok");
    }
    pub fn position(universe: &mut Universe, line: &String) {
        // positionコマンドの読取を丸投げ
        set_position(&line, &mut universe.game);
    }
    pub fn setoption_name(universe: &mut Universe, line: &String) {
        // Example: setoption name USI_Ponder value true
        let label1_width = "setoption name ".len(); // 15
        if let Some(name_width) = line[label1_width..].find(' ') {
            let name = &line[label1_width..(label1_width + name_width)];
            // IO::writeln(&format!("Debug name=|{}|", name));
            let label2_width = " value ".len(); // 7
            let value = &line[(label1_width + name_width + label2_width)..];
            // IO::writeln(&format!("Debug value=|{}|", value));
            match name {
                "DepthNotToGiveUp" => {
                    universe.option_depth_not_to_give_up = value.parse().unwrap();
                }
                "MaxDepth" => {
                    universe.option_max_depth = value.parse().unwrap();
                }
                "MinThinkSec" => {
                    universe.option_min_think_sec = value.parse().unwrap();
                }
                "MaxThinkSec" => {
                    universe.option_max_think_sec = value.parse().unwrap();
                }
                _ => {}
            }
        };
    }
    pub fn usi() {
        let engine_file = EngineFile::read();
        // const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        const VERSION: &'static str = "B4";
        Beam::shoot(&format!("id name {} {}", engine_file.engine.name, VERSION));
        Beam::shoot(&format!("id author {}", engine_file.engine.author));

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
        // アルファベット順ではなく、将棋所のダイアログボックスが見やすくなるように並べろだぜ☆（＾～＾）
        // 読みの深さ関連☆（＾～＾）
        Beam::shoot("option name DepthNotToGiveUp type spin default 4 min 0 max 8");
        Beam::shoot("option name MaxDepth type spin default 7 min 0 max 15");
        // 思考時間関連☆（＾～＾）
        Beam::shoot("option name MinThinkSec type spin default 5 min 0 max 599");
        Beam::shoot("option name MaxThinkSec type spin default 17 min 1 max 600");
        // 評価値関連☆（＾～＾）
        // Beam::shoot(
        //     "option name KomawariWeightPer1000 type spin default 1000 min -100000 max 100000",
        // );
        // Beam::shoot("option name ManyWaysPer1000 type spin default 1000 min -100000 max 100000");
        // Beam::shoot(
        //     "option name PromotionWeightPer1000 type spin default 1000 min -100000 max 100000",
        // );
        Beam::shoot("usiok");
    }
    pub fn usinewgame(universe: &mut Universe) {
        universe.game.clear();
    }
}
