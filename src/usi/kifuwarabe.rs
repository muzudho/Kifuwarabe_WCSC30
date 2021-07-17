use crate::config::*;
use crate::entities::cosmic::recording::Phase;
use crate::entities::cosmic::universe::Universe;
use crate::entities::law::usi::*;
use crate::entities::spaceship::equipment::Beam;
use crate::position::to_move_code;
use crate::search::iterative_deepening_search;
use crate::search::SearchStack;
use crate::usi::Kifuwarabe;
use crate::view::print_info;
use std::cmp::min;

impl Kifuwarabe {
    /// bestmoveコマンドを送るぜ☆（＾～＾） 思考するのもこの中だぜ☆（＾～＾）
    pub fn go(universe: &mut Universe, tokens: &Vec<&str>) {
        // # Example
        //
        // ```
        // go btime 60000 wtime 50000 byoyomi 10000
        // .  .     2     .     4     .       6
        //
        // go btime 40000 wtime 50000 binc 10000 winc 10000
        // .  .     2     .     4     .    6     .    8
        // ```

        // パース
        let (btime, wtime, byoyomi, binc, winc) = {
            if 8 <= tokens.len() && tokens[5] == "binc" {
                // フィッシャー・クロック・ルール
                (
                    tokens[2].parse::<u64>().unwrap(),
                    tokens[4].parse::<u64>().unwrap(),
                    0,
                    tokens[6].parse::<u64>().unwrap(),
                    tokens[8].parse::<u64>().unwrap(),
                )
            } else if 6 <= tokens.len() {
                // 秒読みルール
                (
                    tokens[2].parse::<u64>().unwrap(),
                    tokens[4].parse::<u64>().unwrap(),
                    tokens[6].parse::<u64>().unwrap(),
                    0,
                    0,
                )
            } else {
                (0, 0, 0, 0, 0)
            }
        };

        // 時間管理
        let think_sec = {
            let (think_sec, inc_sec) = match universe.game.history.get_phase() {
                Phase::First => ((btime + byoyomi + binc) / 1000, binc / 1000),
                Phase::Second => ((wtime + byoyomi + winc) / 1000, winc / 1000),
            };

            if universe.game.one_move_sec == 0 {
                // 対局開始時に設定
                // 130手で使いつくす想定（＾～＾）
                universe.game.one_move_sec = think_sec / 130;
            }
            let mut think_sec = min(universe.game.one_move_sec, think_sec);

            if 0 < inc_sec {
                // フィッシャー・クロック・ルール
                // 最低でも （加算時間-1秒）は使おう
                if think_sec < inc_sec - 1 {
                    think_sec = inc_sec - 1;
                }
            } else {
                // 最低でも 1秒は使おう
                if think_sec < 1 {
                    think_sec = 1;
                }
            }

            think_sec
        };

        let mut search_stack = SearchStack::new(universe.option_depth_not_to_give_up);
        let (node_value, bestmove) =
            iterative_deepening_search(universe, &mut search_stack, think_sec);
        // その手を選んだ理由☆（＾～＾）
        print_info(
            &mut universe.game.info,
            None,
            Some((search_stack.state_nodes, search_stack.nps())),
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
    pub fn position(universe: &mut Universe, tokens: &Vec<&str>) {
        // positionコマンドの読取を丸投げ
        set_position(&mut universe.game, tokens);
    }
    pub fn setoption_name(universe: &mut Universe, tokens: &Vec<&str>) {
        // # Example:
        //
        // ```
        // setoption name USI_Ponder value true
        // ```
        let name = tokens[2];
        if 5 <= tokens.len() {
            let value = tokens[4];
            match name {
                "DepthNotToGiveUp" => {
                    universe.option_depth_not_to_give_up = value.parse().unwrap();
                }
                "MaxDepth" => {
                    universe.option_max_depth = value.parse().unwrap();
                }
                // "MinThinkSec" => {
                //     universe.option_min_think_sec = value.parse().unwrap();
                // }
                // "MaxThinkSec" => {
                //     universe.option_max_think_sec = value.parse().unwrap();
                // }
                _ => {}
            }
        }
    }
    pub fn usi() {
        let engine_file = EngineFile::read();
        // const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        const VERSION: &'static str = "B6";
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
